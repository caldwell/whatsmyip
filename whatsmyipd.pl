#!/usr/bin/perl
#  Copyright (c) 2012 David Caldwell -*- cperl -*-

use Mojolicious::Lite;

get '/' => sub {
  my $self = shift;
  app->log->info("IP: ".$self->tx->remote_address);
  $self->render(text => $self->tx->remote_address, format => "text");
};

app->start;

