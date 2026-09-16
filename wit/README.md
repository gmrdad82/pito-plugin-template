<!-- This page owns the rules for the public mirror of the host WIT worlds. -->
# WIT interface mirror

This directory mirrors the host-owned `pito:host` world and each desk's
world byte for byte. Change an interface in its owning repository, never
in this copy.

The upstream drift check will compare this directory with the host's
interfaces. No `.wit` file belongs here until those interfaces land.
