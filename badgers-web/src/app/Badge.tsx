type Props = {
    label: string
    status: string
    color?: string
}

export default function Badge({ label, status, color }: Props) {
    const buildUrl = () => {
        const baseUrl = 'https://honey.badgers.space/badge'
        const params = [
            label,
            status,
            color,
        ]

        return `${baseUrl}/${params.filter(Boolean).join('/')}`
    }

    return (
        <div className="">
            <img loading="lazy" src={buildUrl()} alt={label} />
        </div>
    )
}
