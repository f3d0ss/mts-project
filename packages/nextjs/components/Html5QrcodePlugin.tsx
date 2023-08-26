import { useEffect } from "react";
import { Html5QrcodeScanner, QrcodeErrorCallback, QrcodeSuccessCallback } from "html5-qrcode";
import { Html5QrcodeScannerConfig } from "html5-qrcode/esm/html5-qrcode-scanner";

const qrcodeRegionId = "html5qr-code-full-region";

type Html5QrcodePluginProps = {
  fps: number;
  qrbox?: number;
  aspectRatio?: number;
  disableFlip?: boolean;
  verbose?: boolean;
  qrCodeSuccessCallback: QrcodeSuccessCallback;
  qrCodeErrorCallback?: QrcodeErrorCallback;
};

// Creates the configuration object for Html5QrcodeScanner.
const createConfig = (props: Html5QrcodePluginProps): Html5QrcodeScannerConfig => {
  const config: Html5QrcodeScannerConfig = { fps: undefined };
  if (props.fps) {
    config.fps = props.fps;
  }
  if (props.qrbox) {
    config.qrbox = props.qrbox;
  }
  if (props.aspectRatio) {
    config.aspectRatio = props.aspectRatio;
  }
  if (props.disableFlip !== undefined) {
    config.disableFlip = props.disableFlip;
  }
  return config;
};

const Html5QrcodePlugin = (props: Html5QrcodePluginProps) => {
  useEffect(() => {
    // when component mounts
    const config = createConfig(props);
    const verbose = props.verbose === true;

    const html5QrcodeScanner = new Html5QrcodeScanner(qrcodeRegionId, config, verbose);
    html5QrcodeScanner.render(props.qrCodeSuccessCallback, props.qrCodeErrorCallback);

    // cleanup function when component will unmount
    return () => {
      console.count("unmount");
      html5QrcodeScanner.clear().catch(error => {
        console.error("Failed to clear html5QrcodeScanner. ", error);
      });
    };
    // from doc https://github.com/scanapp-org/html5-qrcode-react
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return <div id={qrcodeRegionId} />;
};

export default Html5QrcodePlugin;
