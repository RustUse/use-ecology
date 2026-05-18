param(
    [switch]$DryRun
)

$ErrorActionPreference = "Stop"

$tools = @(
    "cargo-deny",
    "cargo-audit",
    "cargo-cyclonedx",
    "release-plz",
    "cargo-machete"
)

function Invoke-CommandLine {
    param(
        [Parameter(Mandatory = $true)]
        [string]$FilePath,

        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]]$Arguments
    )

    if ($DryRun) {
        Write-Host "+ $FilePath $($Arguments -join ' ')"
        return
    }

    & $FilePath @Arguments
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "cargo is required"
}

Invoke-CommandLine rustup component add rustfmt clippy

foreach ($tool in $tools) {
    Write-Host "Installing or updating $tool"
    Invoke-CommandLine cargo install --locked $tool
}

Write-Host "Optional RustUse development tools are ready."