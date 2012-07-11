#!/usr/bin/perl
#  Copyright (c) 2012 David Caldwell,  All Rights Reserved. -*- cperl -*-

use Mojolicious::Lite;

get '/' => sub {
  my $self = shift;
  $self->render(text => $self->tx->remote_address);
};

app->start;

