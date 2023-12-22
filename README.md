OBM - OpenCurve Project Build Manager
---

OBM is a tool for help you build opencurve project easily.

Installation
---

```
$ bash -c "$(curl -fsSL https://obm.nos-eastchina1.126.net/script/install.sh)"
```

Usage
---

```bash
$ git clone git@github.com:opencurve/curve.git
$ cd curve
$ obm create
$ make build
```

```
$ vim .obm.cfg
container_name: curve-build-playground.ubuntu20
container_image: opencurvedocker/curve-build:ubuntu20
```



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
| `debian` | `10` | opencurvedocker/curveadm-build:debian10 |

Image for Deploy
---

curve
===

| Version | Image |
| :--- | :--- |
| v2.7 | |
| v2.6 | |
