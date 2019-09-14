import * as compositor from "compositor";

let initialised = false;
let videoElement = undefined;
let input = {
  canvas: undefined,
  ctx: undefined
};

const handleCameraSuccess = (stream /*: MediaStream */) => {
  if ("srcObject" in videoElement) {
    videoElement.srcObject = stream;
  } else {
    videoElement.src = window.URL.createObjectURL(stream);
  }

  this.videoEl.play();
};

const handleCameraError = (e /* : any */) => {
  console.error(e);
};

const initCamera = () => {
  videoElement = document.createElement("video");
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
  compositor.copy();
  compositor.render();
};

const tick = () => {
  init();
  requestAnimationFrame(tick);
  update();
};

tick();
