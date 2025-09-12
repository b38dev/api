import Router from '@koa/router'
import { items } from 'bangumi-data'
// Array.from(document.querySelectorAll('.week ul.coverList .info > p:first-child a')).map(e=>Number(e.href.split('/').pop())).sort((a,b)=>a-b).map((n,i,list)=>(i?n-list[i-1]:n).toString(36)).join(',')
// Array.from(document.querySelectorAll('#cloumnSubjectInfo div.infoWrapper_tv div.infoWrapper')).map(e=>Number(e.id.split('_').pop())).sort((a,b)=>a-b).map((n,i,list)=>(i?n-list[i-1]:n).toString(36)).join(',')

export default function () {
    const router = new Router()
    const map = new Map()
    items.forEach(({ sites }, i) => {
        for (const { site, id } of sites)
            if (site == 'bangumi') return map.set(Number(id), i)
    })

    const parseList = (q?: string, t?: string) => {
        if (!q) return []
        if (t != '1') return q.split(',').map(n => parseInt(n))
        const list = q.split(',').map(n => parseInt(n, 36))
        for (let i = 1; i < list.length; i++) list[i] += list[i - 1]
        return list
    }
    router.get('/', async ctx => {
        const { q, t } = ctx.query
        ctx.body = {
            data: parseList(q as string, t as string).map(id => {
                if (!map.has(id)) return [id, null]
                const data = items[map.get(id)]
                return [id, data]
            }),
        }
    })
    return router.routes()
}
