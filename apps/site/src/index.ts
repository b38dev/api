import Koa from 'koa'
import Router from '@koa/router'
import cors from '@koa/cors'
import onair from '@b38dev/onair'

const router = new Router()
router.use('/onair', onair())
new Koa()
    .use(cors())
    .use(router.routes())
    .listen({ host: '0.0.0.0', port: 4080 })
