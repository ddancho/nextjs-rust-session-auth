import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";
import { validateUser } from "./actions/auth";

const protectedRoutes = ["/logout", "/api/user"];
const freeAccessRoutes = ["/register", "/login"];

export async function middleware(request: NextRequest) {
  try {
    const user = await validateUser();
    const pathname = request.nextUrl.pathname;

    if (user && freeAccessRoutes.includes(pathname)) {
      return NextResponse.redirect(new URL("/", request.url));
    }

    if (!user && protectedRoutes.includes(pathname)) {
      return NextResponse.redirect(new URL("/", request.url));
    }

    return NextResponse.next();
  } catch {
    return NextResponse.redirect(new URL("/", request.url));
  }
}

export const config = {
  matcher: ["/register", "/login", "/logout", "/api/user"],
};
