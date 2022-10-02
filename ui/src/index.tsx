import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
import {ApolloClient, ApolloProvider, InMemoryCache} from "@apollo/client";
import {RootContextProvider} from "./context/app";


const client = new ApolloClient({
  uri: "https://index.space-pussy-1.cybernode.ai/v1/graphql",
  cache: new InMemoryCache(),
})

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);


root.render(
  <React.StrictMode>
    <RootContextProvider>
      <ApolloProvider client={client}>
        <App/>
      </ApolloProvider>
    </RootContextProvider>
  </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();