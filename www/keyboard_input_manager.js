import {downloadFile} from './network';
export default class KeyboardInputManager {
    constructor()
    {
        this.events = {};

        if (window.navigator.msPointerEnabled) {
            // Internet Explorer 10 style
            this.eventTouchstart = 'MSPointerDown';
            this.eventTouchmove = 'MSPointerMove';
            this.eventTouchend = 'MSPointerUp';
        } else {
            this.eventTouchstart = 'touchstart';
            this.eventTouchmove = 'touchmove';
            this.eventTouchend = 'touchend';
        }

        this.delay = 100;
        this.weightsUrl = null;

        const progressContainer = document.querySelector('.progress-container');
        this.showNTupleFooter = () => {
            progressContainer.style.display = 'flex';
        };
        this.hideNTupleFooter = () => {
            progressContainer.style.display = 'none';
        };

        this.sliderHandler();
        this.listen();
    }

    on(event, callback)
    {
        if (!this.events[event]) {
            this.events[event] = [];
        }
        this.events[event].push(callback);
    }

    emit(event, data)
    {
        const callbacks = this.events[event];
        if (callbacks) {
            callbacks.forEach(callback => callback(data));
        }
    }

    listen()
    {
        const map = {
            38: 0,  // Up
            39: 1,  // Right
            40: 2,  // Down
            37: 3,  // Left
            75: 0,  // Vim up
            76: 1,  // Vim right
            74: 2,  // Vim down
            72: 3,  // Vim left
            87: 0,  // W
            68: 1,  // D
            83: 2,  // S
            65: 3   // A
        };

        // Respond to direction keys
        document.addEventListener('keydown', event => {
            const modifiers = event.altKey || event.ctrlKey || event.metaKey ||
                event.shiftKey;
            const mapped = map[event.which];

            if (!modifiers) {
                if (mapped !== undefined) {
                    event.preventDefault();
                    this.emit('move', mapped);
                }
            }

            // R key restarts the game
            if (!modifiers && event.which === 82) {
                this.restart.call(this, event);
            }
        });

        // Respond to button presses
        this.bindButtonPress('.retry-button', this.restart);
        this.bindButtonPress('.restart-button', this.restart);
        this.bindButtonPress('.keep-playing-button', this.keepPlaying);
        this.bindButtonPress('.random-move-button', this.randomMove);

        // Respond to swipe events
        const gameContainer =
            document.getElementsByClassName('game-container')[0];
        // continue converting to ES6 classes

        gameContainer.addEventListener(this.eventTouchstart, event => {
            if ((!window.navigator.msPointerEnabled &&
                 event.touches.length > 1) ||
                event.targetTouches.length > 1) {
                return;  // Ignore if touching with more than 1 finger
            }

            if (window.navigator.msPointerEnabled) {
                touchStartClientX = event.pageX;
                touchStartClientY = event.pageY;
            } else {
                touchStartClientX = event.touches[0].clientX;
                touchStartClientY = event.touches[0].clientY;
            }

            event.preventDefault();
        });

        gameContainer.addEventListener(this.eventTouchmove, event => {
            event.preventDefault();
        });

        gameContainer.addEventListener(this.eventTouchend, event => {
            if ((!window.navigator.msPointerEnabled &&
                 event.touches.length > 0) ||
                event.targetTouches.length > 0) {
                return;  // Ignore if still touching with one or more fingers
            }

            let touchEndClientX, touchEndClientY;

            if (window.navigator.msPointerEnabled) {
                touchEndClientX = event.pageX;
                touchEndClientY = event.pageY;
            } else {
                touchEndClientX = event.changedTouches[0].clientX;
                touchEndClientY = event.changedTouches[0].clientY;
            }

            const dx = touchEndClientX - touchStartClientX;
            const absDx = Math.abs(dx);

            const dy = touchEndClientY - touchStartClientY;
            const absDy = Math.abs(dy);

            if (Math.max(absDx, absDy) > 10) {
                // (right : left) : (down : up)
                this.emit(
                    'move',
                    absDx > absDy ? (dx > 0 ? 1 : 3) : (dy > 0 ? 2 : 0));
            }
        });
    };
    restart(event)
    {
        event.preventDefault();
        this.emit('restart');
    };
    keepPlaying(event)
    {
        event.preventDefault();
        this.emit('keepPlaying');
    };

    randomMove(event)
    {
        event.preventDefault();
        this.emit('randomMove');
    };

    bindButtonPress(selector, fn)
    {
        const button = document.querySelector(selector);
        button.addEventListener('click', fn.bind(this));
        button.addEventListener(this.eventTouchend, fn.bind(this));
    };

    handleDropdownEvent(callback)
    {
        const dropdownContent = document.querySelector('.dropdown-content');
        const dropdownBtn = document.querySelector('.dropbtn');

        dropdownBtn.addEventListener('mouseenter', function() {
            dropdownContent.style.display = 'block';
        });
        dropdownBtn.addEventListener('click', function() {
            dropdownContent.style.display = 'block';
        });

        dropdownContent.addEventListener('click', event => {
            // Check if the clicked element is a dropdown item (an <a> tag)
            if (event.target.tagName === 'A') {
                // Prevent the default action (e.g., navigating to a URL)
                event.preventDefault();

                // Get the id of the selected item
                const selectedItem = event.target.id;
                console.log('selected ', selectedItem);
                callback(selectedItem);

                // dropdownContent.classList.remove('show');
                dropdownContent.style.display = 'none';
            }
        });
    };


    setWeightsUrl(url)
    {
        this.weightsUrl = url;
    }

    shakeAgentsButton()
    {
        // Get the button element
        let button = document.querySelector('.dropbtn');

        console.log('Shaking the agents button');
        // Add event listener to trigger the shaking effect
        // Add the CSS class to apply the shake animation
        button.classList.add('shake-animation');

        // After a short delay, remove the CSS class to stop the animation
        setTimeout(function() {
            button.classList.remove('shake-animation');
        }, 500);  // Duration of the shake animation (0.5s)
    }

    shakeProgressBar()
    {
        let bar = document.querySelector('.progress-container');

        console.log('Shaking the progress bar');
        bar.classList.add('shake-animation');

        setTimeout(function() {
            bar.classList.remove('shake-animation');
        }, 500);  // Duration of the shake animation (0.5s)
    }

    setSelectedMode(mode)
    {
        // Get the dropdown button element
        var dropdownButton = document.querySelector('.dropbtn');

        // Set the text of the dropdown button to the selected mode but
        // capitalize the first letter

        let text = 'Select';
        switch (mode) {
        case 'montecarlo':
            text = 'Monte Carlo';
            break;
        case 'expectimax':
            text = 'Expectimax';
            break;
        case 'ntuple':
            text = 'N-Tuple';
            if (this.weightsUrl === null) {
                console.error('Weights URL is not set');
                this.shakeAgentsButton();
            }
            break;
        case 'random':
            text = 'Random';
            break;
        }

        dropdownButton.textContent = text;
    }

    activationButtonOn()
    {
        // Get the activation button element
        var button = document.querySelector('.random-move-button');

        // Set the button text to 'Deactivate'
        button.textContent = 'Deactivate';
    }

    activationButtonOff()
    {
        // Get the activation button element
        var button = document.querySelector('.random-move-button');

        // Set the button text to 'Deactivate'
        button.textContent = 'Activate';
    }

    // toggleActivationButton()
    // {
    //     // Get the activation button element
    //     var button = document.querySelector('.random-move-button');
    //
    //     // Toggle the button text between 'Activate' and 'Deactivate'
    //     if (button.textContent === 'Activate') {
    //         button.textContent = 'Deactivate';
    //     } else {
    //         button.textContent = 'Activate';
    //     }
    // }



    getSelectedDelay()
    {
        return this.delay;
    }

    sliderHandler()
    {
        // Get the range input element
        let delayRange = document.getElementById('delay-range');
        // Get the label element
        let delayLabel = document.getElementById('delay-label');

        // Add event listener to the range input
        delayRange.addEventListener('input', () => {
            // Update the label text content with the new value of the range
            // pad value with spaces to the left
            delayLabel.textContent = 'Delay: ' + delayRange.value + ' ms';
            this.delay = delayRange.value;
        });
    }

    downloadWeights()
    {
        let progressBar = document.getElementById('download-bar');
        let progressText = document.getElementById('download-progress-label');
        return downloadFile(
            this.weightsUrl, (receivedLength, contentLength) => {
                this.setDownloadProgress(
                    receivedLength, contentLength, progressBar, progressText);
            });
    }

    setDownloadProgress(
        receivedLength, contentLength, progressBar, progressText)
    {
        const receivedLengthMB = Math.round(receivedLength / 1024 / 1024);
        const contentLengthMB = Math.round(contentLength / 1024 / 1024);

        progressBar.style.width =
            (100.0 * receivedLength / contentLength) + '%';
        progressText.textContent =
            `Downloading Weights (${receivedLengthMB}/${contentLengthMB} MB): `;

        if (receivedLength == contentLength) {
            let progressColor = document.querySelector('.color');
            progressColor.style.backgroundColor = 'green';
        }
    }
}
