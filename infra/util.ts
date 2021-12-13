import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import * as pulumi from "@pulumi/pulumi";

export function deployment(name: string, pod: kx.PodBuilder, namespace: string) {

    return new kx.Deployment(name, {
        metadata: {
            name: name,
            namespace: namespace
        },
        spec: pod.asDeploymentSpec({ replicas: 1 })
    })
}

export function service(name: string, deployment: kx.Deployment, namespace: string, port: number, targetPort: number) {

    new k8s.core.v1.Service(name, {
        spec: {
            selector: {
                app: deployment.metadata.name,
            },
            ports: [{
                port: port,
                targetPort: targetPort,
            }],
            type: "ClusterIP",
        },
        metadata: {
            name: name,
            namespace: namespace
        }
    });
}
