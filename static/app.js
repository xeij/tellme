// app.js - Frontend logic for tellme web interface
// Handles content fetching, typewriter animation, and user interactions

// ============================================
// STATE MANAGEMENT
// ============================================

let currentContent = null;
let typewriterInterval = null;
let currentCharIndex = 0;
let startTime = null;
let isTyping = false;

// ============================================
// DOM ELEMENTS
// ============================================

const elements = {
    loading: document.getElementById('loading'),
    contentDisplay: document.getElementById('content-display'),
    errorDisplay: document.getElementById('error-display'),
    periodBadge: document.getElementById('period-badge'),
    wordCount: document.getElementById('word-count'),
    contentTitle: document.getElementById('content-title'),
    typewriterText: document.getElementById('typewriter-text'),
    cursor: document.getElementById('cursor'),
    skipBtn: document.getElementById('skip-btn'),
    nextBtn: document.getElementById('next-btn'),
    retryBtn: document.getElementById('retry-btn'),
    errorMessage: document.getElementById('error-message'),
    statsContent: document.getElementById('stats-content'),
    readingProgress: document.getElementById('reading-progress'),
};

// ============================================
// API FUNCTIONS
// ============================================

async function fetchRandomContent() {
    const response = await fetch('/api/content/random');
    if (!response.ok) {
        throw new Error(`Failed to fetch content: ${response.statusText}`);
    }
    return await response.json();
}

async function recordInteraction(contentId, fullyRead, readingTimeSeconds) {
    try {
        await fetch(`/api/content/${contentId}/interaction`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                content_id: contentId,
                fully_read: fullyRead,
                reading_time_seconds: readingTimeSeconds,
            }),
        });
    } catch (error) {
        console.error('Failed to record interaction:', error);
    }
}

async function fetchStats() {
    try {
        const response = await fetch('/api/stats');
        const stats = await response.json();
        elements.statsContent.textContent = `📚 ${stats.total_content} historical stories available`;
    } catch (error) {
        console.error('Failed to fetch stats:', error);
        elements.statsContent.textContent = 'Stats unavailable';
    }
}

// ============================================
// TYPEWRITER EFFECT
// ============================================

function startTypewriter(text) {
    isTyping = true;
    currentCharIndex = 0;
    elements.typewriterText.textContent = '';
    elements.cursor.classList.remove('hidden');
    elements.skipBtn.classList.remove('hidden');
    elements.nextBtn.classList.add('hidden');
    
    // Adaptive typing speed based on content length
    const speed = text.length > 1000 ? 15 : text.length > 500 ? 25 : 35;
    
    typewriterInterval = setInterval(() => {
        if (currentCharIndex < text.length) {
            elements.typewriterText.textContent += text[currentCharIndex];
            currentCharIndex++;
            
            // Update progress bar
            const progress = (currentCharIndex / text.length) * 100;
            elements.readingProgress.style.width = `${progress}%`;
        } else {
            finishTypewriter(true);
        }
    }, speed);
}

function skipTypewriter() {
    if (!isTyping || !currentContent) return;
    
    clearInterval(typewriterInterval);
    elements.typewriterText.textContent = currentContent.content;
    elements.readingProgress.style.width = '100%';
    finishTypewriter(false);
}

function finishTypewriter(fullyAnimated) {
    isTyping = false;
    clearInterval(typewriterInterval);
    elements.cursor.classList.add('hidden');
    elements.skipBtn.classList.add('hidden');
    elements.nextBtn.classList.remove('hidden');
    
    // Record interaction if fully animated
    if (fullyAnimated && currentContent) {
        const readingTime = Math.floor((Date.now() - startTime) / 1000);
        recordInteraction(currentContent.id, true, readingTime);
    }
}

// ============================================
// CONTENT DISPLAY
// ============================================

async function loadNewContent() {
    try {
        // Record interaction with previous content if skipped
        if (currentContent && isTyping) {
            const readingTime = Math.floor((Date.now() - startTime) / 1000);
            await recordInteraction(currentContent.id, false, readingTime);
        }
        
        // Show loading state
        showLoading();
        
        // Fetch new content
        currentContent = await fetchRandomContent();
        startTime = Date.now();
        
        // Display content
        displayContent(currentContent);
        
    } catch (error) {
        showError(error.message);
    }
}

function displayContent(content) {
    // Update UI elements
    elements.periodBadge.textContent = content.topic;
    elements.wordCount.textContent = `${content.word_count} words`;
    elements.contentTitle.textContent = content.title;
    
    // Show content display
    elements.loading.classList.add('hidden');
    elements.errorDisplay.classList.add('hidden');
    elements.contentDisplay.classList.remove('hidden');
    
    // Start typewriter animation
    startTypewriter(content.content);
}

// ============================================
// UI STATE MANAGEMENT
// ============================================

function showLoading() {
    elements.loading.classList.remove('hidden');
    elements.contentDisplay.classList.add('hidden');
    elements.errorDisplay.classList.add('hidden');
}

function showError(message) {
    elements.errorMessage.textContent = message;
    elements.loading.classList.add('hidden');
    elements.contentDisplay.classList.add('hidden');
    elements.errorDisplay.classList.remove('hidden');
}

// ============================================
// EVENT HANDLERS
// ============================================

elements.skipBtn.addEventListener('click', () => {
    skipTypewriter();
});

elements.nextBtn.addEventListener('click', () => {
    loadNewContent();
});

elements.retryBtn.addEventListener('click', () => {
    loadNewContent();
});

// Keyboard shortcuts
document.addEventListener('keydown', (e) => {
    switch (e.key) {
        case ' ':
        case 'Enter':
            e.preventDefault();
            if (isTyping) {
                skipTypewriter();
            } else if (currentContent) {
                loadNewContent();
            }
            break;
        case 'Escape':
            if (isTyping) {
                skipTypewriter();
            }
            break;
    }
});

// ============================================
// INITIALIZATION
// ============================================

async function init() {
    // Fetch stats
    await fetchStats();
    
    // Load first content
    await loadNewContent();
}

// Start the app when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}
