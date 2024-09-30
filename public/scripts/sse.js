(function() {
    let reconnectAttempts = 0;
    const maxReconnectAttempts = 3;
    const reconnectDelay = 100; // 100 milliseconds
    let evtSource = null;
    let reconnectTimeout = null;

    function setupSSE() {
        evtSource = new EventSource("/sse");

        evtSource.onopen = function() {
            console.log("SSE connection established.");
            reconnectAttempts = 0;
        };

        evtSource.onmessage = function(event) {
            console.log("SSE message received:", event.data);
            reconnectAttempts = 0;
        };

        evtSource.onerror = function(err) {
            console.error("SSE connection error:", err);
            evtSource.close();

            if (reconnectAttempts < maxReconnectAttempts) {
                reconnectAttempts++;
                console.log(`Attempting to reconnect (${reconnectAttempts}/${maxReconnectAttempts})...`);
                reconnectTimeout = setTimeout(setupSSE, reconnectDelay);
            } else {
                console.log("Max reconnect attempts reached. Preparing for reload...");
                checkServerAndReload();
            }
        };
    }

    function checkServerAndReload() {
        const checkInterval = 500; // Check every 500ms
        const maxCheckAttempts = 10; // Try for 5 seconds (10 * 500ms)
        let checkAttempts = 0;

        function check() {
            fetch('/sse', { method: 'HEAD' })
                .then(response => {
                    if (response.ok) {
                        console.log("Server is back online. Reloading...");
                        location.reload();
                    } else {
                        retryCheck();
                    }
                })
                .catch(() => retryCheck());
        }

        function retryCheck() {
            checkAttempts++;
            if (checkAttempts < maxCheckAttempts) {
                setTimeout(check, checkInterval);
            } else {
                console.log("Server still not reachable. Please check your connection.");
                // Optionally, you can show a user-friendly message here
                // instead of automatically reloading
            }
        }

        check();
    }

    // Start the SSE connection when the page loads
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', setupSSE);
    } else {
        setupSSE();
    }
})();
