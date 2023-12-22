docker-opencurve -  Docker tooling for OpenCurve project
---

docker-opencurve is Docker tooling for OpenCurve projecet.

Image for Build
---

curve
===

| Distro | Version | Image |
| :--- | :--- | :--- |
| `ubuntu` | `22` | opencurvedocker/curve-build:ubuntu22 |
|  | `20` | opencurvedocker/curve-build:ubuntu20 |
| `debian` | `11` | opencurvedocker/curve-build:debian11 |
|  | `9` | opencurvedocker/curve-build:debian9 |
| `cenots` | `7` | opencurvedocker/curve-build:centos7 |

curveadm
===

| Distro | Version | Image |
| :--- | :--- | :--- |
| `debian` | `10` | opencurvedocker/curveadm-build:golang1.19 |


Image for Deploy
---

CurveFS
===

| Version | Image | Description |
| :--- | :--- | :--- |
| [v2.7](https://github.com/opencurve/curve/commits/release2.7) | quay.io/opencurve/curve/curvefs:v2.7.0-rc2_b125418 | Hadoop SDK supported |
| [v2.6](https://github.com/opencurve/curve/commits/release2.6-hotfix) | quay.io/opencurve/curve/curvefs:v2.6.0-hotfix_c72cb85 | CTO(close-to-open) performance |

CurveBS
===

| Version | Image | Description |
| :--- | :--- | :--- |
| [v1.2.7](https://github.com/opencurve/curve/commits/release1.2/) | quay.io/opencurve/curve/curvebs:v1.2.7-rc3_47a5267 | Latest stable |
