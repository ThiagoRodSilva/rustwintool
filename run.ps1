# Define a URL do seu executável (ex: GitHub Releases)
$url = "https://github.com/ThiagoRodSilva/rustwintool/releases/latest/download/rustwintool.exe"
$exePath = "$env:TEMP\rustwintool.exe"

Write-Host "Baixando a ferramenta..." -ForegroundColor Cyan
try {
    Invoke-WebRequest -Uri $url -OutFile $exePath -UseBasicParsing
    
    Write-Host "Executando..." -ForegroundColor Green
    # Executa o arquivo e espera ele fechar
    Start-Process -FilePath $exePath -Wait -NoNewWindow
    
    Write-Host "Limpeza concluída." -ForegroundColor Gray
    Remove-Item $exePath -Force
} catch {
    Write-Error "Falha ao baixar ou executar: $_"
}
