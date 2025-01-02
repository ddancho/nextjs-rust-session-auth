import { ASSERT, isEnvValueNotEmptyString } from "@/asserts";
import { MaxAge, MySessionCookie, Path, SameSite } from "@/types";
import { cookies } from "next/headers";

export async function setSessionCookieOnResponse(
  setCookiesHeader: string[]
): Promise<boolean> {
  // server did not send set-cookie header in the response header
  if (setCookiesHeader.length === 0) {
    return false;
  }

  ASSERT(
    isEnvValueNotEmptyString(process.env.SESSION_COOKIE_NAME),
    "SESSION_COOKIE_NAME throws error!"
  );
  const sessionCookieName = process.env.SESSION_COOKIE_NAME;

  let mySessionCookieInfo: string | null = null;

  // check for our session cookie
  for (const setCookie of setCookiesHeader) {
    const pairs = setCookie.trim().split(";");

    for (const pair of pairs) {
      const [name] = pair.split("=");
      if (name === sessionCookieName) {
        mySessionCookieInfo = setCookie;
        break;
      }
    }
  }

  if (!mySessionCookieInfo) {
    // server did send set-cookie header back,
    // but our session cookie is missing
    return false;
  }

  // session cookie is send
  // setting up
  const mySessionCookie: Partial<MySessionCookie> = {};

  const pairs = mySessionCookieInfo.trim().split(";");

  for (const pair of pairs) {
    const [name, value] = pair.split("=").map((s) => s.trim());

    if (name === sessionCookieName) {
      mySessionCookie.name = sessionCookieName;
      mySessionCookie.value = value;
    }

    if (name === "HttpOnly") {
      mySessionCookie.httpOnly = true;
    }

    if (name === "SameSite") {
      mySessionCookie.sameSite = (
        typeof value === "string" ? value.toLowerCase() : value
      ) as SameSite;
    }

    if (name === "Path") {
      mySessionCookie.path = value as Path;
    }

    if (name === "Max-Age") {
      mySessionCookie.maxAge = parseInt(value) as MaxAge;
    }
  }

  if (mySessionCookie.name && mySessionCookie.value) {
    const cookieList = await cookies();

    cookieList.set({
      name: mySessionCookie.name,
      value: mySessionCookie.value,
      httpOnly: mySessionCookie.httpOnly,
      sameSite: mySessionCookie.sameSite,
      path: mySessionCookie.path,
      maxAge: mySessionCookie.maxAge,
    });
  } else {
    // just in case some parsing errors
    return false;
  }

  return true;
}

export async function deleteSessionCookieOnResponse(
  setCookiesHeader: string[]
): Promise<boolean> {
  // server did not send set-cookie header in the response header
  if (setCookiesHeader.length === 0) {
    return false;
  }

  ASSERT(
    isEnvValueNotEmptyString(process.env.SESSION_COOKIE_NAME),
    "SESSION_COOKIE_NAME throws error!"
  );
  const sessionCookieName = process.env.SESSION_COOKIE_NAME;

  let mySessionCookieInfo: string | null = null;

  // check for our session cookie
  for (const setCookie of setCookiesHeader) {
    const pairs = setCookie.trim().split(";");

    for (const pair of pairs) {
      const [name] = pair.split("=");
      if (name === sessionCookieName) {
        mySessionCookieInfo = setCookie;
        break;
      }
    }
  }

  if (!mySessionCookieInfo) {
    // server did send set-cookie header back,
    // but our session cookie is missing
    return false;
  }

  const cookieList = await cookies();

  // have to to delete our session cookie now,
  // session is cleared on the server
  cookieList.delete(sessionCookieName);

  return true;
}
