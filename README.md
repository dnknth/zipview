# A simple CGI to browse ZIP archives

This project started as a coding exercise in the [Rust](https://www.rust-lang.org) programming language. As such, the code style is probably far from perfect.

It provides a simple [CGI](https://de.wikipedia.org/wiki/Common_Gateway_Interface) executable to browse ZIP archives on a HTTP server. The ZIP content is presented like a regular directory, and archive members can be viewed or downloaded. If an `index.html` file is found in a zipped directory, it is shown instead of the generic directory listing.

No content is ever written to disk, all listing and unzipping operations are done in memory. Therefore, even as a CGI, Zip browsing is reasonably fast.

## Example configuration for [Apache](http://httpd.apache.org/docs/2.4/en/)

```apache
ScriptAlias /zipview /usr/lib/cgi-bin/zipview
<Directory "/usr/lib/cgi-bin">
    Require all granted
    Options +ExecCGI
</Directory>

<Directory "/archives/">
    Options Indexes MultiViews SymLinksIfOwnerMatch
    AllowOverride All
    Require all granted
    Action application/zip /zipview
</Directory>
```

Additionally, an XSLT style sheet is expected in `/zipview.xslt` for pretty directory listings. See [htdocs](htdocs/) for an example.
