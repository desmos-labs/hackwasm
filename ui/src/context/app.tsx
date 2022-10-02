import React, {useEffect, useState} from "react";
import {CyberClient} from "@cybercongress/cyber-js";
import {DesmosClient} from "@desmoslabs/desmjs";

export interface Props {
  children?: React.ReactNode
}

export interface RootContextValue {
  cyberJs: null | CyberClient,
  desmosClient: null | DesmosClient
}

const rootContextValue: RootContextValue = {
  cyberJs: null,
  desmosClient: null,
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

  useEffect(() => {
    (async () => {
      const client = await DesmosClient.connect("https://rpc.morpheus.desmos.network");
      setContextValue((old) => {
        return {
          ...old,
          desmosClient: client
        }
      })
    })()
  }, []);


  return <RootContext.Provider children={children} value={value} />
}

