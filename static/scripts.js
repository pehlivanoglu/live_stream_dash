document.addEventListener("DOMContentLoaded", () => {
    const videoPlayer = document.getElementById('videoPlayer');
    const player = dashjs.MediaPlayer().create();
    player.initialize(videoPlayer, null, false);
    player.updateSettings({
        streaming: {
            delay: {
                liveDelay: 1
            }
        }
    });

    const videoUrl = "../video/manifest.mpd";
    initializeVideoSource(videoUrl, player);

    addControlEvent('backward', () => player.seek(player.time() - 10));
    addControlEvent('forward', () => player.seek(player.time() + 10));
    addControlEvent('live', () => player.seek(player.duration() - 2));
    addControlEvent('screenshot', () => takeScreenshot(videoPlayer));
});

async function initializeVideoSource(url, player) {
    try {
        while (true) {
            const response = await fetch(url);
            if (response.ok) {
                player.attachSource(url);
                player.play();
                break;
            }
            await new Promise(resolve => setTimeout(resolve, 1500));
        }
    } catch (error) {
        console.error("Error fetching video source: ", error);
    }
}

function addControlEvent(elementId, action) {
    const element = document.getElementById(elementId);
    element.addEventListener('click', action);
}

function takeScreenshot(videoPlayer) {
    const canvas = document.createElement('canvas');
    canvas.width = videoPlayer.videoWidth;
    canvas.height = videoPlayer.videoHeight;
    const ctx = canvas.getContext('2d');
    ctx.drawImage(videoPlayer, 0, 0, canvas.width, canvas.height);
    const dataURL = canvas.toDataURL('image/png');
    const link = document.createElement('a');
    link.href = dataURL;
    link.download = 'screenshot.png';
    link.click();
}
