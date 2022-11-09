import React from "react";
import { useParams } from "react-router-dom";

export default function NotFound() {
  let { any } = useParams();

  // I'm redirecting users to the linux.ucla.edu not-found site
  // because I think their site is really cool (it has the fortune command
  // being piped into the cowsay command)
  window.location.href = `https://linux.ucla.edu/tswift-not-found/${any}`;


  // The return statement will never be executed since the user will
  // have been redirected to the linux.ucla.edu not-found site.
  return (
    <>
      <div>
        Page not found!
      </div>
    </>
  );
}
