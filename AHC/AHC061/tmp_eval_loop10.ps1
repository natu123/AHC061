param(
    [ValidateSet('QualityFirst', 'Balanced', 'BreadthFirst')]
    [string]$Mode = 'QualityFirst',
    [int]$LoopCount = 3,
    [int]$QuickSeedTo = 19,
    [int]$FullSeedTo = 99,
    [string[]]$CandidateIds = @('x42', 'x47', 'x48', 'x49', 'x38', 'x39', 'x40', 'x41'),
    [string[]]$ExploitCandidateIds = @(),
    [string[]]$ExploreFreshIds = @(),
    [string[]]$ExplorePairIds = @(),
    [string[]]$ExploreBlendIds = @(),
    [int]$FullCandidateLimit = 3
)

$ErrorActionPreference = 'Stop'

$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
if (-not $scriptRoot) {
    $scriptRoot = (Get-Location).Path
} else {
    $scriptRoot = (Resolve-Path $scriptRoot).Path
}
$normalized = $scriptRoot.ToLower()
$invalidPath = '[\\/]projects[\\/]ahc([\\/]|$)'
$expectedPath = '[\\/]atcoder[\\/]ahc[\\/]ahc061$'
if ($normalized -match $invalidPath -or $normalized -notmatch $expectedPath) {
    throw "workspace sanity check failed: expected script execution under '*\\AtCoder\\AHC\\AHC061', current='$scriptRoot'."
}

$tools = Join-Path $scriptRoot 'N52XwIfp_windows/tools_x86_64-pc-windows-gnu'
$tester = Join-Path $tools 'tester.exe'
$inDir = Join-Path $tools 'in'
$solverDir = Join-Path $scriptRoot 'solver/target/release'

$modeConfig = @{
    QualityFirst = @{
        ModeWeight = 0.85
        EfficiencyWeight = 0.15
        FullLimit = 2
        Allocation = @{
            Exploit = 0.05
            ExploreFresh = 0.30
            ExplorePair = 0.50
            ExploreBlend = 0.20
        }
        QualityMean = 0.99
        QualityMedian = 0.985
        QualityMin = 0.95
        NoveltyMean = 0.95
        NoveltyMedian = 0.92
        NoveltyMin = 0.85
        NoveltyScoreMin = 0.12
        NoveltyScale = 0.40
        CompetitionMeanDrop = 0.012
        CompetitionSelectionDrop = 0.06
        CompetitionCap = 4
        FallbackMeanRatio = 0.985
    }
    Balanced = @{
        ModeWeight = 0.70
        EfficiencyWeight = 0.20
        FullLimit = 2
        Allocation = @{
            Exploit = 0.15
            ExploreFresh = 0.25
            ExplorePair = 0.45
            ExploreBlend = 0.30
        }
        QualityMean = 0.985
        QualityMedian = 0.975
        QualityMin = 0.93
        NoveltyMean = 0.94
        NoveltyMedian = 0.90
        NoveltyMin = 0.84
        NoveltyScoreMin = 0.10
        NoveltyScale = 0.40
        CompetitionMeanDrop = 0.02
        CompetitionSelectionDrop = 0.10
        CompetitionCap = 5
        FallbackMeanRatio = 0.985
    }
    BreadthFirst = @{
        ModeWeight = 0.55
        EfficiencyWeight = 0.25
        FullLimit = 3
        Allocation = @{
            Exploit = 0.05
            ExploreFresh = 0.20
            ExplorePair = 0.40
            ExploreBlend = 0.35
        }
        QualityMean = 0.975
        QualityMedian = 0.965
        QualityMin = 0.90
        NoveltyMean = 0.93
        NoveltyMedian = 0.88
        NoveltyMin = 0.82
        NoveltyScoreMin = 0.08
        NoveltyScale = 0.40
        CompetitionMeanDrop = 0.03
        CompetitionSelectionDrop = 0.14
        CompetitionCap = 6
        FallbackMeanRatio = 0.980
    }
}

function Assert-ValidPath {
    param([string]$Path)
    if (-not (Test-Path $Path)) {
        throw "required path not found: $Path"
    }
}

function Get-SolverTable {
    $table = [ordered]@{}
    Get-ChildItem -Path $solverDir -Filter 'x*.exe' | Sort-Object Name | ForEach-Object {
        if ($_.BaseName -match '^x(\d{2})_') {
            $nn = [int]$matches[1]
            if ($nn -ge 4) {
                $key = ('x{0:D2}' -f $nn)
                $table[$key] = $_.FullName
            }
        }
    }
    if (-not $table.Contains('x04')) {
        throw 'x04 baseline is missing from solver/target/release'
    }
    $table
}

function Eval-Set {
    param(
        [string]$Name,
        [string]$BinPath,
        [int]$SeedTo
    )

    $scores = New-Object 'System.Collections.Generic.List[int64]'
    $elapsed = New-Object 'System.Collections.Generic.List[int64]'

    for ($s = 0; $s -le $SeedTo; $s++) {
        $seed = '{0:D4}' -f $s
        $inFile = Join-Path $inDir ($seed + '.txt')
        $errFile = Join-Path $env:TEMP ("tmp_eval_loop10_{0}_{1}.err" -f $Name, $seed)
        $outFile = Join-Path $env:TEMP ("tmp_eval_loop10_{0}_{1}.out" -f $Name, $seed)

        $sw = [System.Diagnostics.Stopwatch]::StartNew()
        $p = Start-Process -FilePath $tester -ArgumentList ('"' + $BinPath + '"') -RedirectStandardInput $inFile -RedirectStandardOutput $outFile -RedirectStandardError $errFile -NoNewWindow -Wait -PassThru
        $sw.Stop()
        $elapsed.Add($sw.ElapsedMilliseconds)

        if ($p.ExitCode -ne 0 -and $p.ExitCode -ne $null) {
            throw "tester failed: $Name seed=$seed exit=$($p.ExitCode)"
        }

        $line = Get-Content $errFile | Select-Object -First 1
        if ($line -notmatch 'Score\s*=\s*([0-9]+)') {
            throw "score parse failed: $Name seed=$seed line='$line'"
        }
        $scores.Add([int64]$matches[1])
        Remove-Item $outFile -Force -ErrorAction SilentlyContinue
        Remove-Item $errFile -Force
    }

    $sorted = $scores.ToArray() | Sort-Object
    $mean = [math]::Round((($scores | Measure-Object -Sum).Sum) / $scores.Count, 1)
    $median = if (($scores.Count % 2) -eq 1) {
        $sorted[[int]($scores.Count / 2)]
    } else {
        ($sorted[($scores.Count / 2) - 1] + $sorted[$scores.Count / 2]) / 2
    }

    [PSCustomObject]@{
        Name = $Name
        SeedTo = $SeedTo
        Mean = $mean
        Median = [int64]$median
        Min = [int64]$sorted[0]
        Max = [int64]$sorted[-1]
        MeanSeedMs = [math]::Round(($elapsed | Measure-Object -Average).Average, 1)
        ElapsedMs = [int64]($elapsed | Measure-Object -Sum).Sum
    }
}

function Register-CandidateIds {
    param(
        [string[]]$Inputs,
        [string]$DefaultClass,
        [hashtable]$Buckets,
        [hashtable]$ClassMap,
        [hashtable]$Seen
    )

    foreach ($entry in $Inputs) {
        if ([string]::IsNullOrWhiteSpace($entry)) {
            continue
        }
        foreach ($piece in ($entry -split ',')) {
            $trimmed = $piece.Trim()
            if ([string]::IsNullOrWhiteSpace($trimmed)) {
                continue
            }
            $id = $trimmed
            $class = $DefaultClass
            if ($trimmed -match '^(x\d+)(?::([A-Za-z]+))?$') {
                $id = $matches[1]
                if ($matches[2]) {
                    switch ($matches[2].ToLower()) {
                        { $_ -in @('x', 'exploit', 'quality') } { $class = 'Exploit'; break }
                        { $_ -in @('n', 'fresh', 'novel', 'explorefresh') } { $class = 'ExploreFresh'; break }
                        { $_ -in @('nn', 'pair', 'pairnovel', 'explorepair') } { $class = 'ExplorePair'; break }
                        { $_ -in @('ne', 'blend', 'mix', 'exploreblend') } { $class = 'ExploreBlend'; break }
                    }
                }
            }

            if (-not $id.StartsWith('x')) { continue }
            if (-not $Seen.ContainsKey($id)) {
                $Seen[$id] = $true
                $Buckets[$class] += ,$id
                $ClassMap[$id] = $class
            }
        }
    }
}

function Build-AllocationCaps {
    param(
        [hashtable]$Cfg,
        [int]$Limit
    )
    $exploitCap = [int][Math]::Floor($Limit * $Cfg.Allocation.Exploit)
    $exploreCap = $Limit - $exploitCap
    $freshCap = [int][Math]::Floor($exploreCap * $Cfg.Allocation.ExploreFresh)
    $pairCap = [int][Math]::Floor($exploreCap * $Cfg.Allocation.ExplorePair)
    $blendCap = [int][Math]::Floor($exploreCap * $Cfg.Allocation.ExploreBlend)
    $assigned = $freshCap + $pairCap + $blendCap

    $blendCap += ($exploreCap - $assigned)
    if ($blendCap -lt 0) {
        $pairCap += $blendCap
        $blendCap = 0
    }
    if ($pairCap -lt 0) {
        $freshCap += $pairCap
        $pairCap = 0
    }
    if ($freshCap -lt 0) {
        $freshCap = 0
    }

    @{
        Exploit = [Math]::Max(0, $exploitCap)
        ExploreFresh = [Math]::Max(0, $freshCap)
        ExplorePair = [Math]::Max(0, $pairCap)
        ExploreBlend = [Math]::Max(0, $blendCap)
    }
}

function Pick-Class {
    param(
        [string]$Primary,
        [hashtable]$Caps,
        [hashtable]$Counts
    )

    $order = switch ($Primary) {
        'Exploit' { @('Exploit', 'ExplorePair', 'ExploreFresh', 'ExploreBlend') }
        'ExploreFresh' { @('ExploreFresh', 'ExplorePair', 'ExploreBlend', 'Exploit') }
        'ExplorePair' { @('ExplorePair', 'ExploreFresh', 'ExploreBlend', 'Exploit') }
        'ExploreBlend' { @('ExploreBlend', 'ExplorePair', 'ExploreFresh', 'Exploit') }
        default { @('ExplorePair', 'ExploreFresh', 'ExploreBlend', 'Exploit') }
    }
    foreach ($c in $order) {
        if (($Counts[$c] + 1) -le $Caps[$c]) {
            return $c
        }
    }
    return $null
}

Assert-ValidPath -Path $tools
Assert-ValidPath -Path $tester
Assert-ValidPath -Path $inDir
Assert-ValidPath -Path $solverDir

$bins = Get-SolverTable
foreach ($name in $bins.Keys) {
    if (-not (Test-Path $bins[$name])) {
        throw "binary not found: $($bins[$name])"
    }
}

$candidateBuckets = @{
    Exploit = @()
    ExploreFresh = @()
    ExplorePair = @()
    ExploreBlend = @()
    Unclassified = @()
}
$candidateClass = @{}
$seen = @{}

Register-CandidateIds -Inputs $ExploitCandidateIds -DefaultClass 'Exploit' -Buckets $candidateBuckets -ClassMap $candidateClass -Seen $seen
Register-CandidateIds -Inputs $ExploreFreshIds -DefaultClass 'ExploreFresh' -Buckets $candidateBuckets -ClassMap $candidateClass -Seen $seen
Register-CandidateIds -Inputs $ExplorePairIds -DefaultClass 'ExplorePair' -Buckets $candidateBuckets -ClassMap $candidateClass -Seen $seen
Register-CandidateIds -Inputs $ExploreBlendIds -DefaultClass 'ExploreBlend' -Buckets $candidateBuckets -ClassMap $candidateClass -Seen $seen
Register-CandidateIds -Inputs $CandidateIds -DefaultClass 'Unclassified' -Buckets $candidateBuckets -ClassMap $candidateClass -Seen $seen

$candidateIdsToRun = @()
foreach ($bucket in @('Exploit', 'ExploreFresh', 'ExplorePair', 'ExploreBlend', 'Unclassified')) {
    foreach ($id in $candidateBuckets[$bucket]) {
        if ($bins.Contains($id) -and $id -ne 'x04') {
            $candidateIdsToRun += $id
        } elseif ($id -ne 'x04') {
            Write-Warning ("candidate not found and ignored: {0}" -f $id)
            $candidateClass.Remove($id) | Out-Null
        }
    }
}

$candidateIdsToRun = $candidateIdsToRun | Sort-Object -Unique
if ($candidateIdsToRun.Count -eq 0) {
    $candidateIdsToRun = @($bins.Keys | Where-Object { $_ -ne 'x04' } | Sort-Object)
    foreach ($id in $candidateIdsToRun) {
        $candidateClass[$id] = 'ExplorePair'
    }
    Write-Output 'candidateIds was empty after filtering, so fallback to all available xNN candidates.'
}

foreach ($id in $candidateIdsToRun) {
    if (-not $candidateClass.ContainsKey($id)) {
        $candidateClass[$id] = 'ExplorePair'
    }
}

$cfg = $modeConfig[$Mode]
$fullSelectionLimit = [Math]::Min($cfg.FullLimit, [Math]::Max(1, $FullCandidateLimit))
$effectiveCompetitionCap = [Math]::Min($cfg.CompetitionCap, $fullSelectionLimit)
$caps = Build-AllocationCaps -Cfg $cfg -Limit $fullSelectionLimit
$exploreCap = $caps.ExploreFresh + $caps.ExplorePair + $caps.ExploreBlend
$exploreTotal = [Math]::Max(1, $exploreCap)

Write-Output ("=== adaptive loop start ({0}) ===" -f (Get-Date -Format 'yyyy-MM-dd HH:mm:ss'))
Write-Output ("=== mode: {0} / quality_weight={1} / efficiency_weight={2} / quick seed=0..{3} / full seed=0..{4} / loop={5} / candidates={6} ===" -f $Mode, $cfg.ModeWeight, $cfg.EfficiencyWeight, $QuickSeedTo, $FullSeedTo, $LoopCount, ($candidateIdsToRun -join ','))
Write-Output ("=== Allocation plan: Exploit={0:P0}, Explore={1:P0} (Fresh={2:P0}/Pair={3:P0}/Blend={4:P0}) / caps E/F/P/B={5}/{6}/{7}/{8} ===" -f ((1 - $exploreCap / [Math]::Max(1, $fullSelectionLimit)), ($exploreCap / [Math]::Max(1, $fullSelectionLimit)), ($caps.ExploreFresh / $exploreTotal), ($caps.ExplorePair / $exploreTotal), ($caps.ExploreBlend / $exploreTotal), $caps.Exploit, $caps.ExploreFresh, $caps.ExplorePair, $caps.ExploreBlend))

$baselineQuick = Eval-Set -Name 'x04' -BinPath (Resolve-Path $bins['x04']) -SeedTo $QuickSeedTo
$baselineFull = Eval-Set -Name 'x04_full' -BinPath (Resolve-Path $bins['x04']) -SeedTo $FullSeedTo
Write-Output ("Baseline quick (x04): mean=$($baselineQuick.Mean), median=$($baselineQuick.Median), min=$($baselineQuick.Min), max=$($baselineQuick.Max), elapsed=$($baselineQuick.ElapsedMs)ms")
Write-Output ("Baseline full  (x04): mean=$($baselineFull.Mean), median=$($baselineFull.Median), min=$($baselineFull.Min), max=$($baselineFull.Max), elapsed=$($baselineFull.ElapsedMs)ms")

for ($loop = 1; $loop -le $LoopCount; $loop++) {
    Write-Output ('')
    Write-Output ("--- Loop {0}/{1} ---" -f $loop, $LoopCount)

    $quickRaw = @()
    foreach ($name in $candidateIdsToRun) {
        $quickRaw += Eval-Set -Name $name -BinPath (Resolve-Path $bins[$name]) -SeedTo $QuickSeedTo
    }

    $quickScored = foreach ($q in $quickRaw) {
        $meanRatio = $q.Mean / $baselineQuick.Mean
        $medianRatio = $q.Median / $baselineQuick.Median
        $minRatio = $q.Min / $baselineQuick.Min

        $qualityRaw = 0.45 * $meanRatio + 0.35 * $medianRatio + 0.20 * $minRatio
        $qualityScore = [math]::Round([math]::Max(0.0, [math]::Min(1.0, $qualityRaw)), 6)

        $noveltyGap = [math]::Abs(1.0 - $meanRatio) * 0.45 + [math]::Abs(1.0 - $medianRatio) * 0.35 + [math]::Abs(1.0 - $minRatio) * 0.20
        $noveltyRaw = $noveltyGap / $cfg.NoveltyScale
        $noveltyScore = [math]::Round([math]::Max(0.0, [math]::Min(1.0, $noveltyRaw)), 6)

        $gainOverMean = [math]::Max(0.0, $q.Mean - $baselineQuick.Mean)
        $efficiencyRaw = if ($q.MeanSeedMs -gt 0) { $gainOverMean / ($q.MeanSeedMs / 1000.0) } else { 0.0 }

        $selectionScore = [math]::Round($cfg.ModeWeight * $qualityScore + (1 - $cfg.ModeWeight) * $noveltyScore, 6)
        $qualityGate = ($meanRatio -ge $cfg.QualityMean) -and (($medianRatio -ge $cfg.QualityMedian) -or ($minRatio -ge $cfg.QualityMin))
        $noveltyGate = $noveltyScore -ge $cfg.NoveltyScoreMin -and ($meanRatio -ge $cfg.NoveltyMean) -and ($medianRatio -ge $cfg.NoveltyMedian) -and ($minRatio -ge $cfg.NoveltyMin)

        [PSCustomObject]@{
            Name = $q.Name
            Mean = $q.Mean
            Median = $q.Median
            Min = $q.Min
            Max = $q.Max
            MeanSeedMs = $q.MeanSeedMs
            ElapsedMs = $q.ElapsedMs
            MeanRatio = $meanRatio
            QualityScore = $qualityScore
            NoveltyScore = $noveltyScore
            SelectionScore = $selectionScore
            EfficiencyRaw = $efficiencyRaw
            CandidateClass = $candidateClass[$q.Name]
            QualityGate = $qualityGate
            NoveltyGate = $noveltyGate
        }
    }

    $maxEfficiency = [Math]::Max(1e-9, (($quickScored | Select-Object -ExpandProperty EfficiencyRaw | Measure-Object -Maximum).Maximum))
    $quickScored = $quickScored | ForEach-Object {
        $candidate = $_
        $efficiencyScore = [math]::Round([math]::Max(0.0, [math]::Min(1.0, $candidate.EfficiencyRaw / $maxEfficiency)), 6)
        $finalScore = [math]::Round((1 - $cfg.EfficiencyWeight) * $candidate.SelectionScore + $cfg.EfficiencyWeight * $efficiencyScore, 6)
        $candidate | Add-Member -NotePropertyName 'EfficiencyScore' -NotePropertyValue $efficiencyScore -Force
        $candidate | Add-Member -NotePropertyName 'FinalScore' -NotePropertyValue $finalScore -Force
        $candidate | Add-Member -NotePropertyName 'EGRQuick' -NotePropertyValue 0.0 -Force
        $candidate
    }

    $selectedGateScored = $quickScored | Where-Object { $_.QualityGate -or $_.NoveltyGate }
    if ($selectedGateScored.Count -gt 0) {
        foreach ($q in $selectedGateScored) {
            $gain = [math]::Max(0.0, ($q.Mean / $baselineQuick.Mean) - 1.0)
            $costSec = [Math]::Max(1e-6, $q.ElapsedMs / 1000.0)
            $q.EGRQuick = [math]::Round($gain / $costSec, 6)
        }
    }

    $quickDisplay = $quickScored | Sort-Object -Property FinalScore -Descending
    $quickDisplay | Select-Object -First 10 | Format-Table Name, CandidateClass, Mean, Median, Min, Max, MeanRatio, QualityScore, NoveltyScore, EfficiencyScore, FinalScore, MeanSeedMs, ElapsedMs
    Write-Output ("class hit summary: gate_pass_total={0}, egr_top={1}" -f $selectedGateScored.Count, ($selectedGateScored | Sort-Object -Property EGRQuick -Descending | Select-Object -First 1 | ForEach-Object { $_.Name + '(' + $_.EGRQuick + ')' }))

    $counts = @{Exploit = 0; ExploreFresh = 0; ExplorePair = 0; ExploreBlend = 0}
    $selectedForFull = @()
    $selectedUnique = @{}

    foreach ($q in $quickDisplay) {
        if (-not ($q.QualityGate -or $q.NoveltyGate)) { continue }
        if ($selectedForFull.Count -ge $effectiveCompetitionCap) { break }

        $baseClass = if ($q.CandidateClass) { $q.CandidateClass } else { 'ExplorePair' }
        $picked = Pick-Class -Primary $baseClass -Caps $caps -Counts $counts
        if (-not $picked) { continue }

        if (-not $selectedUnique.ContainsKey($q.Name)) {
            $selectedUnique[$q.Name] = $true
            $counts[$picked]++
            $q | Add-Member -NotePropertyName 'AllocatedClass' -NotePropertyValue $picked -Force
            $selectedForFull += $q
        }
    }

    if ($quickDisplay.Count -gt 0 -and $selectedForFull.Count -lt $effectiveCompetitionCap) {
        $best = $quickDisplay[0]
        $competitionThresholdMean = $best.Mean * (1 - $cfg.CompetitionMeanDrop)
        $competitionThresholdScore = $best.FinalScore * (1 - $cfg.CompetitionSelectionDrop)
        $competitive = $quickDisplay | Where-Object {
            $_.Mean -ge $competitionThresholdMean -and
            $_.FinalScore -ge $competitionThresholdScore -and
            -not $selectedUnique.ContainsKey($_.Name) -and
            ($_.QualityGate -or $_.NoveltyGate)
        } | Sort-Object -Property FinalScore -Descending

        foreach ($q in $competitive) {
            if ($selectedForFull.Count -ge $effectiveCompetitionCap) { break }
            $baseClass = if ($q.CandidateClass) { $q.CandidateClass } else { 'ExplorePair' }
            $picked = Pick-Class -Primary $baseClass -Caps $caps -Counts $counts
            if (-not $picked) { continue }
            $selectedUnique[$q.Name] = $true
            $counts[$picked]++
            $q | Add-Member -NotePropertyName 'AllocatedClass' -NotePropertyValue $picked -Force
            $selectedForFull += $q
        }
    }

    if ($selectedForFull.Count -eq 0 -and $quickDisplay.Count -gt 0) {
        $fallback = $quickDisplay | Where-Object { $_.Mean -ge ($cfg.FallbackMeanRatio * $baselineQuick.Mean) } | Select-Object -First 1
        if ($fallback) {
            $fallback | Add-Member -NotePropertyName 'AllocatedClass' -NotePropertyValue $fallback.CandidateClass -Force
            $selectedForFull += $fallback
            Write-Output ("fallback full candidate: {0} (final_score={1}, class={2})" -f $fallback.Name, $fallback.FinalScore, $fallback.CandidateClass)
        }
    }

    $selectedByClass = @{}
    foreach ($item in $selectedForFull) {
        $key = if ($item.AllocatedClass) { $item.AllocatedClass } else { 'Unclassified' }
        if (-not $selectedByClass.ContainsKey($key)) { $selectedByClass[$key] = 0 }
        $selectedByClass[$key]++
    }
    Write-Output ("selected allocation: " + (($selectedByClass.Keys | ForEach-Object { "{0}={1}" -f $_, $selectedByClass[$_] }) -join ', '))

    if ($selectedForFull.Count -eq 0) {
        Write-Output 'selected full candidates: (none)'
        Write-Output ("recap: no high-confidence candidate in loop {0}." -f $loop)
        continue
    }

    $fullNames = @($selectedForFull | Select-Object -ExpandProperty Name | Sort-Object)
    Write-Output ("selected full candidates: {0}" -f ($fullNames -join ','))

    Write-Output ("### full (seed 0..{0}) ###" -f $FullSeedTo)
    $full = @()
    $full += $baselineFull
    foreach ($q in $selectedForFull) {
        $full += Eval-Set -Name ($q.Name + '_full') -BinPath (Resolve-Path $bins[$q.Name]) -SeedTo $FullSeedTo
    }

    $fullSorted = $full | Sort-Object -Property Mean -Descending
    $fullSorted | Format-Table Name, Mean, Median, Min, Max, MeanSeedMs, ElapsedMs

    $baseline = $fullSorted | Where-Object { $_.Name -eq 'x04_full' } | Select-Object -First 1
    $candsFull = $fullSorted | Where-Object { $_.Name -ne 'x04_full' }

    $bestMeanGain = 0.0
    $bestCandidate = $null
    foreach ($r in $candsFull) {
        if (-not $baseline) { break }
        $meanDelta = [math]::Round((($r.Mean - $baseline.Mean) / $baseline.Mean) * 100, 2)
        $medDelta = [math]::Round((($r.Median - $baseline.Median) / $baseline.Median) * 100, 2)
        $minDelta = [math]::Round((($r.Min - $baseline.Min) / $baseline.Min) * 100, 2)
        Write-Output ("{0} vs baseline mean_delta={1}% median_delta={2}% min_delta={3}%" -f $r.Name, $meanDelta, $medDelta, $minDelta)
        if ($meanDelta -gt $bestMeanGain) {
            $bestMeanGain = $meanDelta
            $bestCandidate = $r.Name
        }
    }

    if ($bestMeanGain -ge 50) {
        Write-Output ("recap loop {0}: target candidate candidate={1}, mean_gain={2}%" -f $loop, $bestCandidate, $bestMeanGain)
        break
    } else {
        Write-Output ("recap loop {0}: best full candidate {1} mean_gain={2}% (target +50% not reached)" -f $loop, $bestCandidate, $bestMeanGain)
    }
}
