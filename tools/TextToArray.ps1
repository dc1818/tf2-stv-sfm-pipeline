param(
    [Parameter(Mandatory = $true)]
    [string] $InputPath,

    [Parameter(Mandatory = $true)]
    [string] $ObjectName,

    [Parameter(Mandatory = $true)]
    [string] $OutputPath
)

$ErrorActionPreference = 'Stop'

if (-not [IO.File]::Exists($inputPath)) {
    throw "Input file not found: $InputPath"
}

$bytes = [IO.File]::ReadAllBytes($inputPath)
$output = New-Object Text.StringBuilder
[void] $output.Append("static unsigned char $ObjectName[] = {`r`n    ")

for ($i = 0; $i -lt $bytes.Length; $i++) {
    [void] $output.Append(('0x{0:x2},' -f $bytes[$i]))
    if (($i % 20) -eq 19) {
        [void] $output.Append("`r`n    ")
    }
}

[void] $output.Append("0x00`r`n};`r`n`r`n")
$asciiWithoutBom = New-Object Text.ASCIIEncoding
[IO.File]::WriteAllText($OutputPath, $output.ToString(), $asciiWithoutBom)
