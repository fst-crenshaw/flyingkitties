
//! Note this layout module is not a very good strategy
//! using the rocket_contrib Templates is heavily recommended
//! for anything besides simple applications.
//! 
//! For instance, if you later decide to add any kind of privilege based
//! menu system or conditionally display information, like generation time
//! or even a custom title, Handlebars templates is a good solution. Try it.

use rocket::response::content::Html;

use auth::sanitization::*;
// use auth::authorization::*;

pub fn layout(contents: &str) -> Html<String> {
    // if padding should be added, add it here and change the contents variable name below
    let mut output = String::with_capacity(contents.len() + LAYOUT_HEADER.len() + LAYOUT_FOOTER.len() + 30);
    output.push_str(LAYOUT_HEADER);
    output.push_str(contents);
    output.push_str(LAYOUT_FOOTER);
    Html(output)
}


pub const LAYOUT_HEADER: &'static str = r##"
<!doctype html>
<html lang="en">
    <head>
        <title>Flying Kitties Attacking the Earth</title>
        
        <!-- Required meta tags -->
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">

 <!-- Bootstrap CSS -->
        <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0-beta/css/bootstrap.min.css" integrity="sha384-/Y6pD6FV/Vv2HJnA6t+vslU6fwYXjCFtcEpHbNJ0lyAFsXTsjBbfaDjzALeQsN6M" crossorigin="anonymous">
        
        <!-- Custom CSS -->
        <link id="css-stylesheet" type="text/css" href="css/blogr.css" rel="stylesheet" />
                
        <!-- JavaScript -->
        <script src="sha256.js"></script>
        <script src="login.js"></script>
    </head>
    <body>
        <div id="mainWrapper" class="main-wrapper">
                <div id="body">
                    <div class="container">
                        
"##;

pub const LAYOUT_FOOTER: &'static str = r##"
                    </div>
                </div>
            </div>
            
            
        <!-- </div> -->
        <!-- jQuery first, then Popper.js, then Bootstrap JS -->
        <script src="https://code.jquery.com/jquery-3.2.1.slim.min.js" integrity="sha384-KJ3o2DKtIkvYIK3UENzmM7KCkRr/rE9/Qpg6aAZGJwFDMVNA/GpGFF93hXpG5KkN" crossorigin="anonymous"></script>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.11.0/umd/popper.min.js" integrity="sha384-b/U6ypiBEHpOf/4+1nzFpr53nxSS+GLCkfwBdFNTxtclqqenISfwAzpKaMNFNmj4" crossorigin="anonymous"></script>
        <script src="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0-beta/js/bootstrap.min.js" integrity="sha384-h0AbiXch4ZDo7tp9hKZ4TsHbi047NrKGLO3SEJAg45jXxnGIfYzk4Si90RDIqNm1" crossorigin="anonymous"></script>        
        <!-- Custom JavaScript -->
        <script>
        </script>        
    </body>
</html>
"##;

pub fn layout_form(url: &str) -> String {
    format!(r##"

<div class="kitty">
<pre>
../\„„./\.
.(='•'= ) .
.(") „. (").
. \,\„„/,/			    
<pre>
</div>
                        <form id="needs-validation" action="{url}" name="login_form" method="post" novalidate>
                            <div class="form-group" id="userGroup">
                                <label for="usernameField">Email Address</label>
                                <div class="col-md-9 mb-3">
                                    <input type="text" name="username" value="" class="form-control" id="usernameField" aria-describedby="idHelp" placeholder="Username" required>
                                    <div class="invalid-feedback">
                                        Please specify a username
                                    </div>
                                </div>
                                <!-- <small id="idHelp" class="form-text text-muted">Your email address will not be shared with anyone else.</small> -->
                            </div>
                            <div class="form-group" id="passGroup">
                                <label for="passwordField">Password</label>
                                <div class="col-md-9 mb-3">
                                    <input type="password" name="password" class="form-control" id="passwordField" placeholder="Password" required>
                                    <div class="invalid-feedback">
                                        A password is requierd.
                                    </div>
                                    <input type="password" id="passwordHidden" class="hidden-pass form-control">
                                </div>
                            </div>
                            <div class="submit">
                                <button type="submit" class="btn btn-primary" id="submit-button-id">Login</button>
                            </div>
                        </form>
"##, url=url)
}

pub fn layout_retry_form(url: &str, username: &str) -> String {
    format!(r##"
                        <form id="needs-validation" action="{url}" name="login_form" method="post" novalidate>
                            <div class="form-group" id="userGroup">
                                <label for="usernameField">Email Address</label>
                                <div class="col-md-9 mb-3">
                                    <input type="text" name="username" value="{username}" class="form-control" id="usernameField" aria-describedby="idHelp" placeholder="Username" required>
                                    <div class="invalid-feedback">
                                        Please specify a username
                                    </div>
                                </div>
                                <!-- <small id="idHelp" class="form-text text-muted">Your email address will not be shared with anyone else.</small> -->
                            </div>
                            <div class="form-group" id="passGroup">
                                <label for="passwordField">Password</label>
                                <div class="col-md-9 mb-3">
                                    <input type="password" name="password" class="form-control" id="passwordField" placeholder="Password" required>
                                    <div class="invalid-feedback">
                                        A password is requierd.
                                    </div>
                                    <input type="password" id="passwordHidden" class="hidden-pass form-control">
                                </div>
                            </div>
                            <div class="submit">
                                <button type="submit" class="btn btn-primary" id="submit-button-id">Login</button>
                            </div>
                        </form>
"##, url=url, username=sanitize(username))
}

#[allow(dead_code)]
pub fn alert_danger(msg: &str) -> String {
    format!(r##"
                        <div class="centered-msg alert alert-danger" role="alert">
                            {why}
                        </div>
"##, why=msg)
}
#[allow(dead_code)]
pub fn alert_success(msg: &str) -> String {
    format!(r##"
                        <div class="centered-msg alert alert-success" role="alert">
                            {why}
                        </div>
"##, why=msg)
}
#[allow(dead_code)]
pub fn alert_info(msg: &str) -> String {
    format!(r##"
                        <div class="centered-msg alert alert-info" role="alert">
                            {why}
                        </div>
"##, why=msg)
}
#[allow(dead_code)]
pub fn alert_warning(msg: &str) -> String {
    format!(r##"
                        <div class="centered-msg alert alert-warning" role="alert">
                            {why}
                        </div>
"##, why=msg)
}
#[allow(dead_code)]
pub fn alert_primary(msg: &str) -> String {
    format!(r##"
                        <div class="centered-msg alert alert-primary" role="alert">
                            {why}
                        </div>
"##, why=msg)
}