import * as compositor from "compositor";

let initialised = false;
let cameraInitialised = false;
let videoElement = undefined;
let input = {
  canvas: undefined,
  ctx: undefined
};
const fps = 30;
let fpsInterval, now, old, elapsed;

const handleCameraSuccess = (stream /*: MediaStream */) => {
  if ("srcObject" in videoElement) {
    videoElement.srcObject = stream;
  } else {
    videoElement.src = window.URL.createObjectURL(stream);
  }

  videoElement.play();
  cameraInitialised = true;
};

const handleCameraError = (e /* : any */) => {
  console.error(e);
};

const initCamera = () => {
  videoElement = document.getElementById("camera");
  videoElement.setAttribute("muted", "");
  videoElement.setAttribute("playsinline", "");
  videoElement.setAttribute("autoplay", "");

  const mediaDeviceConstraints /*: any */ = {
    audio: false,
    video: {
      height: {
        exact: 480
      },
      width: {
        exact: 640
      }
    }
  };

  navigator.mediaDevices
    .getUserMedia(mediaDeviceConstraints)
    .then(handleCameraSuccess.bind(this))
    .catch(handleCameraError.bind(this));
};

const initInput = () => {
  input.canvas = document.getElementById("input");
  input.ctx = input.canvas.getContext("2d");
  input.ctx.clearRect(0, 0, input.canvas.width, input.canvas.height);
  input.ctx.fillStyle = "#000000";
  input.ctx.fillRect(0, 0, input.canvas.width, input.canvas.height);
};

const init = () => {
  if (!initialised && typeof compositor.initialise !== "undefined") {
    fpsInterval = 1000 / fps;
    old = Date.now();
    compositor.initialise("output");
    initInput();
    initCamera();
    initialised = true;
  }
};

const copyVideoIntoInputCanvas = () => {
  input.ctx.drawImage(
    videoElement,
    0,
    0,
    input.canvas.width,
    input.canvas.height
  );
};

const update = () => {
  copyVideoIntoInputCanvas();
  if (initialised && cameraInitialised) {
    try {
      compositor.copy(
        input.ctx.getImageData(0, 0, input.canvas.width, input.canvas.height)
      );
    } catch (e) {
      console.log(e);
    }
  }
};

const tick = () => {
  // request another frame
  requestAnimationFrame(tick);
  // calc elapsed time since last loop
  now = Date.now();
  elapsed = now - old;

  // if enough time has elapsed, draw the next frame
  if (elapsed > fpsInterval) {
    old = now - (elapsed % fpsInterval);
    update();
  }
};

init();
tick();
