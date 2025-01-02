"use server";

import { ASSERT, isEnvValueNotEmptyString } from "@/asserts";
import {
  ServerResponse,
  User,
  UserSignInSchema,
  UserSignUpSchema,
} from "@/types";
import { getErrorMessage } from "@/util";
import { cookies } from "next/headers";
import {
  deleteSessionCookieOnResponse,
  setSessionCookieOnResponse,
} from "@/actions/_util";

const stdErrMsg = "Something went wrong. Please try again later.";

export async function register(newUser: unknown): Promise<ServerResponse> {
  const result = UserSignUpSchema.safeParse(newUser);
  let response: ServerResponse;

  if (!result.success) {
    let errorMessage = "";

    result.error.issues.forEach((issue) => {
      errorMessage = errorMessage + issue.path[0] + ": " + issue.message + ". ";
    });

    response = {
      status: "error",
      message: errorMessage,
    };

    return response;
  }

  ASSERT(
    isEnvValueNotEmptyString(process.env.SERVER_ADDRESS),
    "SERVER_ADDRESS throws error!"
  );
  const server = process.env.SERVER_ADDRESS;
  const url = `${server}/api/auth/register`;

  try {
    const res = await fetch(url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(result.data),
    });

    if (!res.ok) {
      console.log("register res status:", res.status);
      console.log("register res text:", await res.text());

      response = {
        status: "error",
        message: stdErrMsg,
      };
      return response;
    }

    response = {
      status: "success",
      message: "User is successfully created",
    };

    return response;
  } catch (error: unknown) {
    response = {
      status: "error",
      message: getErrorMessage(error),
    };

    return response;
  }
}

export async function login(userData: unknown): Promise<ServerResponse> {
  const result = UserSignInSchema.safeParse(userData);
  let response: ServerResponse;

  if (!result.success) {
    let errorMessage = "";

    result.error.issues.forEach((issue) => {
      errorMessage = errorMessage + issue.path[0] + ": " + issue.message + ". ";
    });

    response = {
      status: "error",
      message: errorMessage,
    };

    return response;
  }

  ASSERT(
    isEnvValueNotEmptyString(process.env.SERVER_ADDRESS),
    "SERVER_ADDRESS throws error!"
  );
  const server = process.env.SERVER_ADDRESS;
  const url = `${server}/api/auth/login`;

  try {
    const res = await fetch(url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(result.data),
    });

    if (!res.ok) {
      console.log("login res status:", res.status);
      console.log("login res text:", await res.text());

      response = {
        status: "error",
        message: stdErrMsg,
      };
      return response;
    }

    // setting up session cookie on nextjs ssr
    const setUpCookieResult = await setSessionCookieOnResponse(
      res.headers.getSetCookie()
    );
    if (!setUpCookieResult) {
      response = {
        status: "error",
        message: stdErrMsg,
      };
      return response;
    }

    response = {
      status: "success",
      message: "User is successfully logged in",
    };

    return response;
  } catch (error: unknown) {
    response = {
      status: "error",
      message: getErrorMessage(error),
    };

    return response;
  }
}

export async function logout(): Promise<ServerResponse> {
  let response: ServerResponse;

  ASSERT(
    isEnvValueNotEmptyString(process.env.SESSION_COOKIE_NAME),
    "SESSION_COOKIE_NAME throws error!"
  );
  const sessionCookieName = process.env.SESSION_COOKIE_NAME;

  const cookieList = await cookies();

  const mySessionCookie = cookieList.get(sessionCookieName);
  if (
    mySessionCookie === undefined ||
    !mySessionCookie.name ||
    !mySessionCookie.value
  ) {
    response = {
      status: "error",
      message: stdErrMsg,
    };

    return response;
  }

  ASSERT(
    isEnvValueNotEmptyString(process.env.SERVER_ADDRESS),
    "SERVER_ADDRESS throws error!"
  );
  const server = process.env.SERVER_ADDRESS;
  const url = `${server}/api/auth/logout`;

  try {
    const res = await fetch(url, {
      method: "POST",
      headers: { Cookie: `${mySessionCookie.name}=${mySessionCookie.value}` },
    });

    if (!res.ok) {
      console.log("logout res status:", res.status);
      console.log("logout res text:", await res.text());

      response = {
        status: "error",
        message: stdErrMsg,
      };

      return response;
    }

    // delete session cookie on nextjs ssr
    const deleteCookieResult = await deleteSessionCookieOnResponse(
      res.headers.getSetCookie()
    );

    if (!deleteCookieResult) {
      response = {
        status: "error",
        message: stdErrMsg,
      };
      return response;
    }

    response = {
      status: "success",
      message: "User is successfully logged out",
    };

    return response;
  } catch (error: unknown) {
    console.log("logout err:", getErrorMessage(error));

    response = {
      status: "error",
      message: stdErrMsg,
    };

    return response;
  }
}

export async function validateUser(): Promise<User | null> {
  ASSERT(
    isEnvValueNotEmptyString(process.env.SESSION_COOKIE_NAME),
    "SESSION_COOKIE_NAME throws error!"
  );
  const sessionCookieName = process.env.SESSION_COOKIE_NAME;

  const cookieList = await cookies();

  const mySessionCookie = cookieList.get(sessionCookieName);
  if (
    mySessionCookie === undefined ||
    !mySessionCookie.name ||
    !mySessionCookie.value
  ) {
    return null;
  }

  ASSERT(
    isEnvValueNotEmptyString(process.env.SERVER_ADDRESS),
    "SERVER_ADDRESS throws error!"
  );
  const server = process.env.SERVER_ADDRESS;
  const url = `${server}/api/user`;

  try {
    const res = await fetch(url, {
      method: "GET",
      headers: { Cookie: `${mySessionCookie.name}=${mySessionCookie.value}` },
    });

    if (!res.ok) {
      console.log("getuser res status:", res.status);
      console.log("getuser res text:", await res.text());

      return null;
    }

    // UserDto from server
    const user = (await res.json()) as User;

    return user;
  } catch (error: unknown) {
    console.log("getuser err:", getErrorMessage(error));

    return null;
  }
}
