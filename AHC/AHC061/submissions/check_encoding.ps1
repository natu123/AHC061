$path = 'C:\Users\kenji\projects\AtCoder\AHC\AHC061-2\submissions\submission_x226.rs'
$bytes = [System.IO.File]::ReadAllBytes($path)

# BOM check
if ($bytes.Length -ge 3 -and $bytes[0] -eq 0xEF -and $bytes[1] -eq 0xBB -and $bytes[2] -eq 0xBF) {
    Write-Host "BOM: UTF-8 BOM detected"
} else {
    Write-Host "BOM: No BOM"
}

# NUL byte check
$nulCount = 0
foreach ($b in $bytes) { if ($b -eq 0) { $nulCount++ } }
Write-Host "NUL bytes: $nulCount"

# Line ending check
$crlf = 0; $lf = 0
for ($i = 0; $i -lt $bytes.Length; $i++) {
    if ($bytes[$i] -eq 0x0A) {
        if ($i -gt 0 -and $bytes[$i-1] -eq 0x0D) { $crlf++ } else { $lf++ }
    }
}
Write-Host "CRLF: $crlf, LF-only: $lf"

# Non-ASCII check
$content = [System.IO.File]::ReadAllText($path)
$matches = [regex]::Matches($content, '[^\x00-\x7F]')
Write-Host "Non-ASCII chars: $($matches.Count)"
if ($matches.Count -gt 0) {
    $matches | Select-Object -First 10 | ForEach-Object {
        Write-Host "  char: [$($_.Value)] code: $([int][char]$_.Value) at index $($_.Index)"
    }
}
