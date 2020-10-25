extern crate cgi;
extern crate format_xml;

use std::env;
use std::fs;
use std::io::Read;
use format_xml::xml;


cgi::cgi_main! { |_request: cgi::Request| -> cgi::Response {
    
    let mut path_info = env::var( "PATH_INFO").unwrap();
    let mut path = env::var( "PATH_TRANSLATED").unwrap();
    let mut extra = String::new();

    loop { // Split PATH_TRANSLATED at the Zip file path
        
        if let Ok( md) = fs::metadata( &path) {
            if md.is_file() { // Got it
                break;
            };
        }
        
        // Get all path components
        let mut parts_info = path_info.split( '/').collect::<Vec<_>>();
        let mut parts = path.split( '/').collect::<Vec<_>>();

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
                path_info = parts_info.join( "/");
                path = parts.join( "/");
            },
            None => { // Empty result?
                return cgi::empty_response( 404);
            }
        }
    }
    
    match fs::File::open( &path) {
        Ok( file) => match zip::ZipArchive::new( file) {
            Ok( mut archive) => {
                if extra.len() == 0 { // List archive content
                    match archive.by_name( "index.html") {
                        // Show index.html if present on top level
                        Ok( mut index) => {
                            let mut buffer = Vec::new();
                            return match index.read_to_end( &mut buffer) {
                                Ok( _) => match std::str::from_utf8( &buffer) {
                                    Ok( html) => cgi::html_response( 200, html),
                                    Err( e) => cgi::err_to_500( Err( e)),
                                },
                                Err( e) => cgi::err_to_500( Err( e))
                            };
                        }
                        Err( _) => { // Fall through for XML listing
                        }
                    };
                    
                    // Sort archive contents by lowercase name
                    let mut names : Vec<&str> = Vec::new();
                    for name in archive.file_names() {
                        names.push( name);
                    }
                    names.sort_by( |a, b| a.to_lowercase().cmp( &b.to_lowercase()));
                    
                    // Get name of Zip file
                    let title = path_info.split( '/')
                        .collect::<Vec<_>>().pop().unwrap();
                    
                    // Render XML listing
                    return cgi::binary_response( 200, "text/xml", xml! {
                        <?xml version="1.0" encoding="UTF-8"?>
                        <?xml-stylesheet type="text/xsl" href="/dk/zip.xslt"?>
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
                        
                    }.to_string().as_bytes().to_vec());
                }
                else { // Extract single file from archive
                    if let Ok( mut data) = archive.by_name( &extra) {
                        let mut buffer = Vec::new();
                        if let Ok( _) = data.read_to_end( &mut buffer) {
                            let guess = mime_guess::from_path( &extra);
                            let mime = match guess.first_raw() {
                                Some( mimetype) => mimetype,
                                None => "application/octet-stream"
                            };
                            return cgi::binary_response(
                                200, mime, buffer)
                        }
                    }
                }
            },
            Err( e) => {
                return cgi::err_to_500( Err( e));
            }
        },
        Err( e) => {
            return cgi::err_to_500( Err( e));
        }
    }
    
    let text = format!(
        "<html><body>
         PATH_INFO: {}
         <br>
         PATH_TRANSLATED: {}
         <br>
         EXTRA_PATH: {}
         </body></html>", path_info, path, extra);
    
    cgi::html_response( 200, text)
} }
