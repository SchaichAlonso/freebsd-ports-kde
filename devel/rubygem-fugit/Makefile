# Created by: Po-Chuan Hsieh <sunpoet@FreeBSD.org>
# $FreeBSD$

PORTNAME=	fugit
PORTVERSION=	1.1.6
CATEGORIES=	devel rubygems
MASTER_SITES=	RG

MAINTAINER=	sunpoet@FreeBSD.org
COMMENT=	Time tools for flor and the floraison project

LICENSE=	MIT
LICENSE_FILE=	${WRKSRC}/LICENSE.txt

RUN_DEPENDS=	rubygem-et-orbi>=1.1.6:devel/rubygem-et-orbi \
		rubygem-raabro>=1.1:textproc/rubygem-raabro

NO_ARCH=	yes
USE_RUBY=	yes
USES=		gem

# Disable rdoc
OPTIONS_EXCLUDE=DOCS

.include <bsd.port.mk>
