Overview
========

This repository contains binding libraries for connecting to PulseAudio from the Rust programming
language.

Three bindings are provided:
 * `libpulse_binding` for `libpulse`,
 * `libpulse_simple_binding` for `libpulse-simple`, and
 * `libpulse_glib_binding` for `libpulse-mainloop-glib`.

See the repective library sub-directories for details.

The bindings are based upon the public API of PulseAudio, as provided in the PulseAudio 'include' C
header files. They provide:
 * A basic export of the C API.
 * For much of the API, simpler and safer interfaces to the underlying C functions and data
   structures, for instance providing wrappers for PulseAudio objects with drop trait
   implementations that automatically free the object upon going out of scope, ala smart pointers.

Author
======

These bindings were not produced by the PulseAudio project, they were produced by an independent
developer - Lyndon Brown.

Copyright & Licensing
=====================

All files in this source code repository, except as noted below, are licensed under the GNU Lesser
General Public License (See file LICENSE-LGPL for details).

The files within the 'includes' directory, have been copied directly from the PulseAudio source.
These files are kept for development purposes only (to be compared through diff checking against
future versions to find changes that may need propagating into new versions of this binding library.
To be clear, they are not used in any compilation processes. They are licensed under LGPL.

The binding libraries provided in this source code repository have been built upon the public API of
PulseAudio, as described in the PulseAudio 'include' files, with documentation in particular largely
being copied from those files. These bindings may be considered derivative works under the
PulseAudio license. PulseAudio itself is licensed under LGPL version 2.1+. These bindings, as
specified above, are under that same license.

The logo images files are a combined derivative of the Rust programming language icon, with a core
element of the PulseAudio icon.

Source Code Contents
====================

 - includes/                    - A copy of the original C header files the bindings have been built
                                  to interface with.
 - pulse-binding/               - The main high-level binding library.
 - pulse-binding-mainloop-glib/ - The high-level binding library for the GLIB mainloop.
 - pulse-binding-simple/        - The high-level binding library for the PulseAudio 'simple'
                                  component.
 - pulse-sys/                   - The main raw C API interface library.
 - pulse-sys-mainloop-glib/     - The raw C API interface library for the GLIB mainloop.
 - pulse-sys-simple/            - The raw C API interface library for the PulseAudio 'simple'
                                  component.
 - src/                         - A dummy binary crate, creating a Cargo workspace, depending upon
                                  all library crates, such that they all build efficiently together
                                  to the same target directory (including documentaton).
