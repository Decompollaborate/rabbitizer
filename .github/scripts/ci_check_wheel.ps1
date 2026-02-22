# A Powershell port of `ci_check_wheel.sh`. Refer to that script instead.

# Any change made here should be made in `ci_check_wheel.sh` too.

param (
    [Parameter(Mandatory = $true)]
    [string]$PYTHON_VERSION,

    [Parameter(Mandatory = $true)]
    [string]$KEY,

    [string]$EXTRA
)

# Equivalent to `set -e`
$ErrorActionPreference = "Stop"

if (Test-Path ".venv") {
    Remove-Item -Recurse -Force ".venv"
}

# When $EXTRA is empty powershell passes the argument as an empty argument to
# uv, so we need to explicitly check the argument and only pass it if it is not
# empty to avoid uv from erroring
if ($EXTRA) {
    uv venv --no-config .venv -p $PYTHON_VERSION $EXTRA
} else {
    uv venv --no-config .venv -p $PYTHON_VERSION
}

.\.venv\Scripts\Activate.ps1
uv run --no-config --no-build --no-sync python --version
uv pip install $(Get-ChildItem -Path .\wheelhouse\ -Recurse -Filter "rabbitizer-*-$KEY*")
uv run --no-config --no-build --no-sync python -c "import rabbitizer; print(rabbitizer.Instruction(0).disassemble())"
