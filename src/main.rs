extern crate cgi;
extern crate format_xml;

use std::env;
use std::fs;
use std::io::Read;
use std::io::Seek;
use format_xml::xml;
use zip::ZipArchive;


/// Issue a HTTP redirect to the given `location`.
fn redirect( location: &str) -> cgi::Response {
    cgi::http::response::Builder::new()
        .status( 307)
        .header( cgi::http::header::LOCATION, location)
        .body(vec![])
        .unwrap()
}


/// Extract the file given by `path` from `archive`.
/// Returns a optional HTTP response if `path` is found, else `None`.
fn extract<R: Read + Seek>( archive: &mut ZipArchive<R>, path: &str) -> Option< cgi::Response> {
    match archive.by_name( path) {
        Ok( mut data) => {
            let mut buffer = Vec::new();
            match data.read_to_end( &mut buffer) {
                Ok( _) => {
                    let guess = mime_guess::from_path( path);
                    let mime = match guess.first_raw() {
                        Some( mimetype) => mimetype,
                        None => "application/octet-stream"
                    };
                    Some( cgi::binary_response( 200, mime, buffer))
                }
                Err( e) => Some( cgi::err_to_500( Err( e)))
            }
        }
        Err( _) => None
    }
}


/// Check whether `path` starts with `prefix`
/// and does not contain additional path separators `/`.
fn matches( path: &str, prefix: &str) -> bool {
    if !path.starts_with( prefix) {
        false
    }
    else {
        let path2 = String::from( path.replace( prefix, ""));
        let n = path2.chars().filter( |c| *c == '/').count();
        n == 0 || (n == 1 && path2.ends_with( "/"))
    }
}


/// Produce an XML listing of the archive content.
/// Returns a HTTP response.
fn list<R: Read + Seek>( archive: &mut ZipArchive<R>, title: &str, prefix: &str) -> cgi::Response {

    // Add a trailing slash to directory name `prefix`
    let mut prefix2: String = String::from( prefix);
    if !prefix.is_empty() {
        prefix2.push( '/');
    }
    
    // Get contents of directory
    let mut names : Vec<String> = archive.file_names()
        .filter( |s| matches( s, &prefix2))
        .map( |s| s.replace( &prefix2, ""))
        .collect();

    // Sort contents by lowercase name
    names.sort_by( |a, b| a.to_lowercase().cmp( &b.to_lowercase()));
    
    // Render XML listing
    cgi::binary_response( 200, "text/xml", xml! {
        <?xml version="1.0" encoding="UTF-8"?>
        <?xml-stylesheet type="text/xsl" href="/zipview.xslt"?>
        <zip name={title}>
            for name in (&names) {
                if (name.ends_with( "/")) {
                    <dir>{name}</dir>
                } else {
                    let guess = mime_guess::from_path( name);
                    match( guess.first()) {
                        Some( mimetype) => {
                            <file type={mimetype}>{name}</file>
                        }
                        None => {
                            <file>{name}</file>
                        }
                    }
                }
            }
        </zip>
    }.to_string().as_bytes().to_vec())
}


// Main entry point
cgi::cgi_main! { |_request: cgi::Request| -> cgi::Response {
    
    let url_path = env::var( "PATH_INFO").unwrap();
    let mut url = String::from( &url_path);
    let mut zip_path = env::var( "PATH_TRANSLATED").unwrap();
    let mut extra = String::new();

    loop { // Extract Zip file path from PATH_TRANSLATED
        
        if let Ok( md) = fs::metadata( &zip_path) {
            if md.is_file() { // Got it
                break;
            };
        }
        
        // Get all path components
        let mut parts_info = url.split( '/').collect::<Vec<_>>();
        let mut parts = zip_path.split( '/').collect::<Vec<_>>();

        // Move the last non-empty part into `extra`
        parts_info.pop();
        match parts.pop() {
            Some( last) => {
                if last.len() > 0 {
                    if extra.len() > 0 {
                        extra.insert( 0, '/');
                    }
                    extra.insert_str( 0, &last);
                }
                url = parts_info.join( "/");
                zip_path = parts.join( "/");
            },
            None => { // Empty result?
                return cgi::empty_response( 404);
            }
        }
    }
    
    // Add a trailing slash to the Zip path,
    // we need it for link resolution in `index.html`.
    if extra.len() == 0 {
        let mut location = env::var( "PATH_INFO").unwrap();
        if !location.ends_with( "/") {
            location.push( '/');
            return redirect( &location);
        }
    }

    // Let's go!
    match fs::File::open( &zip_path) {
        Ok( file) => match zip::ZipArchive::new( file) {
            Ok( mut archive) => {
                if url_path.ends_with( "/") { // List archive content
                    // Show index.html if present on top level
                    match extract( &mut archive, "index.html") {
                        Some( index) => { 
                            return index;
                        }
                        None => {} // Return XML listing
                    };
                    list( &mut archive, &url_path, &extra)
                }
                else { // Extract single file from archive
                    extract( &mut archive, &extra).unwrap_or( 
                        cgi::empty_response( 404))
                }
            },
            Err( e) => cgi::err_to_500( Err( e))
        },
        Err( e) => cgi::err_to_500( Err( e))
    }
}}
