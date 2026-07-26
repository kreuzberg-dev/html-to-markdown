import {convert} from "@xberg-io/html-to-markdown";

const visitor = {
  visitAudio(ctx, src) {
    console.log("visitAudio called with src:", src);
    return {custom : "[AUDIO]"};
  },
};

try {
  const result = convert('<audio src="test.mp3"></audio>', {visitor});
  console.log("Result:", result);
} catch (err) {
  console.error("Error:", err);
}
