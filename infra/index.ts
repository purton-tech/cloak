import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import * as pulumi from "@pulumi/pulumi";
import { service, deployment } from './util'

const APP_NAME="app"
const AUTH_NAME="auth"
const WWW_NAME="www"

const NAME_SPACE = 'keyvault'
const DB_URL_SECRET = 'database-urls'
const MIGRATION_DB_URL = 'migrations-database-url'
const APP_DB_URL = 'app-database-url'
const AUTH_DB_URL = 'auth-database-url'
//const READ_ONLY_DB_URL = 'readonly-database-url'
const config = new pulumi.Config();

const envoyPod = new kx.PodBuilder({
    imagePullSecrets: [{ name: 'image-pull' }],
    containers: [{
        name: "envoy",
        image: `ianpurton/keyvault:envoy@${config.require('envoy')}`,
        ports: { http: 7100 }
    },{
        // Argo Tunnel Sidecar
        name: "tunnel",
        image: "ezkrg/cloudflared",
        command: ["cloudflared", "tunnel"],
        args: [
            `--url=http://127.0.0.1:7100`,
            `--hostname=keyvault.authn.tech`,
            "--origincert=/etc/cloudflared/cert.pem",
            "--no-autoupdate"
        ],
        volumeMounts: [{
            name: "tunnel-secret-volume",
            mountPath: "/etc/cloudflared/"
        }]
    }],
    volumes: [{
        name: "tunnel-secret-volume",
        secret: {
            secretName: 'cloudflare-cert-cream',
            items: [
                { key: "cert.pem", path: "cert.pem" }
            ]
        }
    }]
})

const webPod = new kx.PodBuilder({
    imagePullSecrets: [{ name: 'image-pull' }],
    containers: [{
        name: APP_NAME,
        image: `ianpurton/keyvault:app@${config.require('app')}`,
        ports: { http: 7103 },
        env: [
            { name: "APP_DATABASE_URL", 
                valueFrom: {
                    secretKeyRef: {
                        name: DB_URL_SECRET,
                        key: APP_DB_URL
                    }
                }
            }
        ]
    }],
    initContainers: [{
        name: "server-init",
        image: `ianpurton/keyvault:init@${config.require('init')}`,
        imagePullPolicy: 'Always',
        env: [
            { name: "DATABASE_URL", 
                valueFrom: {
                    secretKeyRef: {
                        name: DB_URL_SECRET,
                        key: MIGRATION_DB_URL
                    }
                }
            }
        ]
    }]
})

const wwwPod = new kx.PodBuilder({
    imagePullSecrets: [{ name: 'image-pull' }],
    containers: [{
        name: WWW_NAME,
        image: `ianpurton/keyvault:www@${config.require('www')}`,
        ports: { http: 80 }
    }]
})

const authPod = new kx.PodBuilder({
    imagePullSecrets: [{ name: 'image-pull' }],
    containers: [{
        name: AUTH_NAME,
        image: 'authnproxy/authnproxy:latest',
        imagePullPolicy: 'Always',
        ports: { http: 9090 },
        env: [
            { name: "AUTH_TYPE", value: "encrypted" },
            { name: "DATABASE_URL", 
                valueFrom: {
                    secretKeyRef: {
                        name: DB_URL_SECRET,
                        key: AUTH_DB_URL
                    }
                }
            },
            { name: "SECURE_COOKIE", value: 'true' },
            { name: "REDIRECT_URL", value: '/app/vaults' },
            { name: "FORWARD_URL", value: '127.0.0.1' }, 
            { name: "FORWARD_PORT", value: '8080' },
            { name: "SKIP_AUTH_FOR", value: '/$$,/images/*,/static/*,/contact' },
            { name: "SECRET_KEY", value: config.requireSecret('secret_key') },
        ]
    }]
})

deployment("envoy", envoyPod, NAME_SPACE)
const wwwDeployment = deployment("www", wwwPod, NAME_SPACE)
const authDeployment = deployment("auth", wwwPod, NAME_SPACE)
const appDeployment = deployment("app", wwwPod, NAME_SPACE)

service(APP_NAME, appDeployment, NAME_SPACE, 7103, 7103)
service(WWW_NAME, wwwDeployment, NAME_SPACE, 7104, 80)
service(AUTH_NAME, authDeployment, NAME_SPACE, 9090, 9090)