What's My IP?
=============

Ever find yourself wondering that? Then set up this tiny HTTP handler on a
remote server somewhere and then hit it with your browser or curl.

## Installing

### On Debian or Ubuntu:

1. apt-get install libmojolicious-perl
2. ./whatsmyipd.pl daemon

### Anywhere else:

It varies, but most likely:

1. cpan -i Mojolicious
2. ./whatsmyipd.pl daemon

### Other options

For daemon options (like setting the port):

    ./whatsmyipd.pl daemon --help

For alternate ways to run (CGI, plack, etc.):

    ./whatsmyipd.pl --help

## License

Your choice: MIT, GPL (any version), MPL, Artistic. It's too short for me to care about.

Copyright © 2012 David Caldwell <david@porkrind.org>
