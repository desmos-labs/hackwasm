import {RootContext} from "../context/app";
import React from "react";

export function useRootContext() {
  return React.useContext(RootContext);
}
