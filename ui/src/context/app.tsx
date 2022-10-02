import React, {useEffect, useState} from "react";
import {CyberClient} from "@cybercongress/cyber-js";

export interface Props {
  children?: React.ReactNode
}

export interface RootContextValue {
  cyberJs: null | CyberClient,
}

const rootContextValue: RootContextValue = {
  cyberJs: null
}

export const RootContext = React.createContext(rootContextValue);

export const RootContextProvider: React.FC<Props> = ({ children }) => {
  const [value, setContextValue] = useState(rootContextValue)

  useEffect(() => {
    (async () => {
      const client = await CyberClient.connect("https://rpc.space-pussy-1.cybernode.ai");
      setContextValue((old) => {
        return {
          ...old,
          cyberJs: client
        }
      })
    })()
  }, []);

  return <RootContext.Provider children={children} value={value} />
}

