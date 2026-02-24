# This script checks a given Python wheel inside the `dist` is installable in a
# given Python version.
#
# It recieves the following arguments:
# - The python version to check, it must be compatible with uv.
# - A key value to allow searching for the wheel in the `dist` folder. Only a
#   single wheel in the folder must contain this value in its name.
#   Recommended values: abi3, cp314t, pypy39 and similar values.
# - (OPTIONAL) A single aditional flag to pass to `uv venv`.
#   Usually `--managed-python`.

# Any change made here should be made in `ci_check_wheel.ps1` too.

PYTHON_VERSION=$1
KEY=$2
EXTRA=$3

# Exit with an error value if any command produces an error.
set -e

# We make a venv with the Python version we were told to.
rm -rf .venv
uv venv --no-config -p $PYTHON_VERSION $EXTRA
source .venv/bin/activate
# Allows us to check we are actually using the requested Python version.
uv run --no-config --no-build --no-sync python --version

# We install the wheel by looking it up in the dist folder.
# We need to do a `find` command here because we don't know the exact name of
# the wheel (it can be affected by package version, arch, python version, etc.).
uv pip install $(find ./wheelhouse/ -name "rabbitizer-*-$KEY*")
# Check something basic to make sure it was installed correctly.
uv run --no-config --no-build --no-sync python -c "import rabbitizer; print(rabbitizer.Instruction(0).disassemble())"
