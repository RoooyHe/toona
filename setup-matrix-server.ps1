# Matrix Server Setup Script for Windows
# This script sets up a local Synapse Matrix server using Docker

Write-Host "=== Matrix Server Setup ===" -ForegroundColor Cyan

# Check if Docker is running
Write-Host "`nChecking Docker..." -ForegroundColor Yellow
try {
    docker version | Out-Null
    Write-Host "Docker is running" -ForegroundColor Green
} catch {
    Write-Host "Error: Docker is not running. Please start Docker Desktop first." -ForegroundColor Red
    exit 1
}

# Check if synapse-data directory exists
if (Test-Path "synapse-data") {
    Write-Host "`nsynapse-data directory already exists." -ForegroundColor Yellow
    $response = Read-Host "Do you want to remove it and start fresh? (y/N)"
    if ($response -eq "y" -or $response -eq "Y") {
        Write-Host "Removing existing data..." -ForegroundColor Yellow
        docker-compose down -v 2>$null
        Remove-Item -Recurse -Force synapse-data
        Write-Host "Existing data removed" -ForegroundColor Green
    } else {
        Write-Host "Keeping existing data" -ForegroundColor Green
    }
}

# Generate Synapse configuration if needed
if (-not (Test-Path "synapse-data/homeserver.yaml")) {
    Write-Host "`nGenerating Synapse configuration..." -ForegroundColor Yellow
    
    # Create synapse-data directory
    New-Item -ItemType Directory -Force -Path synapse-data | Out-Null
    
    # Generate config using environment variables (still required in latest version)
    docker run -it --rm `
        -v ${PWD}/synapse-data:/data `
        -e SYNAPSE_SERVER_NAME=localhost `
        -e SYNAPSE_REPORT_STATS=no `
        matrixdotorg/synapse:latest `
        generate
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Configuration generated successfully" -ForegroundColor Green
        
        # Enable registration for easier testing
        Write-Host "Enabling user registration..." -ForegroundColor Yellow
        $configPath = "synapse-data/homeserver.yaml"
        if (Test-Path $configPath) {
            $config = Get-Content $configPath -Raw
            # Enable registration
            if ($config -match '#enable_registration: false') {
                $config = $config -replace '#enable_registration: false', 'enable_registration: true'
            } elseif ($config -match 'enable_registration: false') {
                $config = $config -replace 'enable_registration: false', 'enable_registration: true'
            } else {
                # Add if not present
                $config = $config -replace '(# Registration)', "enable_registration: true`n`$1"
            }
            Set-Content -Path $configPath -Value $config
            Write-Host "User registration enabled" -ForegroundColor Green
        }
    } else {
        Write-Host "Error: Failed to generate configuration" -ForegroundColor Red
        exit 1
    }
}

# Start the server
Write-Host "`nStarting Matrix server..." -ForegroundColor Yellow
docker-compose up -d

if ($LASTEXITCODE -eq 0) {
    Write-Host "Matrix server started successfully" -ForegroundColor Green
    
    # Wait for server to be ready
    Write-Host "`nWaiting for server to be ready..." -ForegroundColor Yellow
    Start-Sleep -Seconds 10
    
    # Check if server is responding
    $maxRetries = 10
    $retryCount = 0
    $serverReady = $false
    
    while ($retryCount -lt $maxRetries -and -not $serverReady) {
        try {
            $response = Invoke-WebRequest -Uri "http://localhost:8008/_matrix/client/versions" -UseBasicParsing -TimeoutSec 2
            if ($response.StatusCode -eq 200) {
                $serverReady = $true
                Write-Host "Server is ready!" -ForegroundColor Green
            }
        } catch {
            $retryCount++
            if ($retryCount -lt $maxRetries) {
                Write-Host "Waiting... ($retryCount/$maxRetries)" -ForegroundColor Yellow
                Start-Sleep -Seconds 3
            }
        }
    }
    
    if (-not $serverReady) {
        Write-Host "Server is taking longer than expected. Check logs with: docker-compose logs -f synapse" -ForegroundColor Yellow
    }
    
    Write-Host "`n=== Next Steps ===" -ForegroundColor Cyan
    Write-Host "1. Create an admin user:"
    Write-Host "   docker exec -it synapse register_new_matrix_user http://localhost:8008 -c /data/homeserver.yaml -a" -ForegroundColor White
    Write-Host "`n2. In Toona, login with:"
    Write-Host "   Server: http://localhost:8008" -ForegroundColor White
    Write-Host "   Username: @your_username:localhost" -ForegroundColor White
    Write-Host "   Password: (the password you set)" -ForegroundColor White
    Write-Host "`n3. View logs:"
    Write-Host "   docker-compose logs -f synapse" -ForegroundColor White
    Write-Host "`n4. Stop server:"
    Write-Host "   docker-compose down" -ForegroundColor White
} else {
    Write-Host "Error: Failed to start Matrix server" -ForegroundColor Red
    Write-Host "Check logs with: docker-compose logs synapse" -ForegroundColor Yellow
    exit 1
}
