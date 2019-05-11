# coolcats2
**Rust rewrite of Clutter/Coolcats, a fully distributed social messaging app built on Holochain**

This is a Clutter clone, as close to exact in functionality as I could make it.

The front-end is implemented in Rust with the Yew framework, and of course the back-end is Rust as well.

See https://github.com/holochain/clutter for the original.

See https://github.com/pythagorean/coolcats for the Python port that was made of this, both for the old Holochain Proto.

Assuming you have already installed the command line `hc` tool and the `holochain` conductor, you should also
make sure you have installed and are using an `8.x LTS version of nodejs`, and the `rust nightly-2019-01-24 toolchain`, along with `yarn` for managing and installing `node` packages (you can configure `yarn` e.g. [here](https://stackoverflow.com/a/40333409). You will also need to run:
    cargo install cargo-web
Also, the [http-server](https://www.npmjs.com/package/http-server) package that can be installed via `yarn`:

    yarn global add http-server

You may also need to install other dependencies such as `fswatch`.

You can then start a multiuser server test, by running:

    make startnet

You should have test instances you can access on http://localhost:8000 and http://localhost:8001, additional instances
can be easily added in the Makefile but performance may drag at this time.

![Coolcats2 landing page](https://i.imgur.com/0nULXbx.jpg)

When you want to stop, just ^C and then to cleanup any unstopped processes, run:

    make stopnet

Not for any sort of production use whatsoever at this time, no warranty express or implied.

## License
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

This code is Copyright (C) 2019 by Michael Goldman to the extent it is a novel implementation, and rights are
assigned to The MetaCurrency Project (Eric Harris-Braun, Arthur Brock, et. al.) to the extent that it is derivative.
Currently this is GPLv3 licensed to all, other licenses are being considered by the project which they may
re-license or fork this code under. The author reserves the right to fork under other licenses as well.
