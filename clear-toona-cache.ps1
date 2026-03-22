# Clear Toona Cache Script
# This script removes all Toona application data and cache

Write-Host "=== Clear Toona Cache ===" -ForegroundColor Cyan

$cachePath = "$env:APPDATA\robius\robrix"

if (Test-Path $cachePath) {
    Write-Host "`nCache directory found: $cachePath" -ForegroundColor Yellow
    Write-Host "This will delete:" -ForegroundColor Yellow
    Write-Host "  - All user sessions and login data" -ForegroundColor White
    Write-Host "  - Local message history" -ForegroundColor White
    Write-Host "  - Encryption keys" -ForegroundColor White
    Write-Host "  - Application settings" -ForegroundColor White
    Write-Host "  - Kanban cache" -ForegroundColor White
    
    $response = Read-Host "`nAre you sure you want to delete all cache? (y/N)"
    
    if ($response -eq "y" -or $response -eq "Y") {
        try {
            Remove-Item -Recurse -Force $cachePath
            Write-Host "`nCache cleared successfully!" -ForegroundColor Green
            Write-Host "You will need to login again when you start Toona." -ForegroundColor Yellow
        } catch {
            Write-Host "`nError: Failed to clear cache" -ForegroundColor Red
            Write-Host $_.Exception.Message -ForegroundColor Red
            Write-Host "`nMake sure Toona is not running and try again." -ForegroundColor Yellow
            exit 1
        }
    } else {
        Write-Host "`nCache clearing cancelled." -ForegroundColor Yellow
    }
} else {
    Write-Host "`nCache directory not found: $cachePath" -ForegroundColor Yellow
    Write-Host "Either Toona has never been run, or the cache is already cleared." -ForegroundColor White
}

Write-Host ""
