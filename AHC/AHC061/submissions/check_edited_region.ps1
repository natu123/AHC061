$path = 'C:\Users\kenji\projects\AtCoder\AHC\AHC061-2\submissions\submission_x226.rs'
$lines = [System.IO.File]::ReadAllLines($path)

# Check around the edited regions (budget change ~line 1450, iterative deepening ~line 1573)
$regions = @(
    @{Name="budget (L1448-1452)"; Start=1447; End=1452},
    @{Name="iter-deepen start (L1573-1610)"; Start=1572; End=1610},
    @{Name="budget x211 (L8709-8713)"; Start=8708; End=8713},
    @{Name="budget x212 (L8973-8977)"; Start=8972; End=8977}
)

$bytes = [System.IO.File]::ReadAllBytes($path)

foreach ($r in $regions) {
    Write-Host "`n=== $($r.Name) ===" -ForegroundColor Cyan
    for ($i = $r.Start; $i -lt [Math]::Min($r.End, $lines.Length); $i++) {
        Write-Host "${i}: $($lines[$i])"
    }
}

# Check for any unusual byte sequences (0x80-0xFF) in edited regions
# Find byte offset of line 1448
Write-Host "`n=== Byte-level check around edits ===" -ForegroundColor Yellow
$lineOffsets = @()
$offset = 0
foreach ($line in $lines) {
    $lineOffsets += $offset
    $offset += [System.Text.Encoding]::UTF8.GetByteCount($line) + 1  # +1 for LF
}

foreach ($r in $regions) {
    $startByte = $lineOffsets[$r.Start]
    $endByte = if ($r.End -lt $lineOffsets.Length) { $lineOffsets[$r.End] } else { $bytes.Length }
    $suspicious = 0
    for ($i = $startByte; $i -lt $endByte; $i++) {
        if ($bytes[$i] -gt 127) { $suspicious++; Write-Host "  HIGH BYTE at offset $i : 0x$($bytes[$i].ToString('X2'))" }
    }
    if ($suspicious -eq 0) {
        Write-Host "$($r.Name): clean (all ASCII)" -ForegroundColor Green
    }
}
