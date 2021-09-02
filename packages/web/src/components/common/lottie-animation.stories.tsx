import React from "react";
import { LottieAnimation } from "../common/lottie-animation";
import animationData1 from "../../assets/lotties/people-morph-flow.json";
import animationData2 from "../../assets/lotties/13805-working-people.json";
import animationData3 from "../../assets/lotties/14583-multi-tasking.json";
import animationData4 from "../../assets/lotties/28654-happy-freelancer.json";
import animationData5 from "../../assets/lotties/40815-building.json";
import animationData6 from "../../assets/lotties/51084-contract-signing.json";
import animationData8 from "../../assets/lotties/loading.json";
import animationData9 from "../../assets/lotties/people-morph-flow.json";
import animationData10 from "../../assets/lotties/woman-flowers.json";
import animationData11 from "../../assets/lotties//59953-error.json";

export default {
  title: "Design System/LottieAnimation",
  component: LottieAnimation,
  parameters: {
    chromatic: { disableSnapshot: true },
  },
};

export const showAnimation1 = () => (
  <LottieAnimation width={400} height={400} data={animationData1} />
);
export const showAnimation2 = () => (
  <LottieAnimation width={400} height={400} data={animationData2} />
);
export const showAnimation3 = () => (
  <LottieAnimation width={400} height={400} data={animationData3} />
);
export const showAnimation4 = () => (
  <LottieAnimation width={400} height={400} data={animationData4} />
);
export const showAnimation5 = () => (
  <LottieAnimation width={400} height={400} data={animationData5} />
);
export const showAnimation6 = () => (
  <LottieAnimation width={400} height={400} data={animationData6} />
);
export const showAnimation8 = () => (
  <LottieAnimation width={400} height={400} data={animationData8} />
);
export const showAnimation9 = () => (
  <LottieAnimation width={400} height={400} data={animationData9} />
);
export const showAnimation10 = () => (
  <LottieAnimation width={400} height={400} data={animationData10} />
);
export const showAnimation11 = () => (
  <LottieAnimation width={400} height={400} data={animationData11} />
);
