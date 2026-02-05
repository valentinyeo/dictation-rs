@echo off
echo ============================================
echo Push Dictation App to GitHub
echo ============================================
echo.

REM Check if git is initialized
if not exist .git (
    echo Initializing git repository...
    git init
    echo.
)

REM Add all files
echo Adding files...
git add .
echo.

REM Commit
echo Committing changes...
git commit -m "Initial commit: Super lean dictation app"
echo.

REM Prompt for GitHub repository URL
echo Please create a new repository on GitHub first!
echo Then enter the repository URL below.
echo Example: https://github.com/yourusername/dictation-rs.git
echo.
set /p REPO_URL="Enter GitHub repository URL: "

REM Add remote if not exists
git remote remove origin 2>nul
git remote add origin %REPO_URL%
echo.

REM Push to GitHub
echo Pushing to GitHub...
git push -u origin main
echo.

echo ============================================
echo Done!
echo.
echo Next steps:
echo 1. Go to your GitHub repository
echo 2. Click the "Actions" tab
echo 3. Wait for the build to complete (~5-10 min)
echo 4. Download the artifact from the completed workflow
echo.
echo See GET_STARTED.md for detailed instructions.
echo ============================================
pause
