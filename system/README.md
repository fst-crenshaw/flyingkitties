# System Requirements

The Rust web application is served by an NGINX webserver.  It's
configuration, on Ubuntu, is located in /etc/ngnix/sites-enabled/flyingkitties.net.conf.

The Rust web application is started on the system by systemd.  It's service
is located at /etc/systemd/system/flyingkitties.net.service.

These files are made available here for posterity.