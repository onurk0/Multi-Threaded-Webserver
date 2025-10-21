This is a mini webserver in Rust used to get a basic understanding of how a server processes HTML requests. 
Use **cargo build** to compile and  **cargo run** to run. To load the webpage, 
enter **http://localhost:8888/** or **127.0.0.1:88888** into your browser.
If you enter **http://localhost:7777** or **127.0.0.1:7777**, you will be sent 
to **http://google.com** via a 301 redirect. 
Any typos in the URI will result in a 404 Error Page to be displayed.
