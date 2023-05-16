import { NextRequest, NextResponse } from "next/server"

interface BadgeOverrides {
    labelColor?: string,
    color?: string,
    theme?: string,
}

export default class BadgeUtil {
    static async generate(label: string, status: string, overrides: BadgeOverrides = {}): Promise<NextResponse> {
        const api = {
            proto: process.env.NEXT_PUBLIC_API_PROTO,
            host: process.env.NEXT_PUBLIC_API_HOST,
        }
        const pathParams = {
            label: encodeURIComponent(label),
            status: encodeURIComponent(status),
        }
        const queryParams = Object
            .entries(overrides)
            .map(([key, value]) => `${key}=${encodeURIComponent(value)}`)
            .join('&')
        const resp = await fetch(`${api.proto}://${api.host}/badge/${pathParams.label}/${pathParams.status}?${queryParams}`)
        const data = await resp.arrayBuffer()
        return new NextResponse(data, {
            status: resp.status,
            statusText: resp.statusText,
            headers: {
                'Content-Type': 'image/svg+xml',
            }
        })
    }

    static async passThrough(request: NextRequest): Promise<NextResponse> {
        const api = {
            proto: process.env.NEXT_PUBLIC_API_PROTO,
            host: process.env.NEXT_PUBLIC_API_HOST,
        }
        const resp = await fetch(`${api.proto}://${api.host}${request.nextUrl.pathname}${request.nextUrl.search}`)
        const data = await resp.arrayBuffer()
        return new NextResponse(data, {
            status: resp.status,
            statusText: resp.statusText,
            headers: {
                'Content-Type': 'image/svg+xml',
            }
        })
    }
}