# Created by: Steve Wills <swills@FreeBSD.org>
# $FreeBSD$

PORTNAME=	minitar-cli
PORTVERSION=	0.6.1
CATEGORIES=	archivers rubygems
MASTER_SITES=	RG

MAINTAINER=	sunpoet@FreeBSD.org
COMMENT=	Library and command-line utility for tar archives

LICENSE=	BSD3CLAUSE RUBY
LICENSE_COMB=	dual
LICENSE_FILE_BSD3CLAUSE=${WRKSRC}/docs/bsdl.txt
LICENSE_FILE_RUBY=	${WRKSRC}/docs/ruby.txt

RUN_DEPENDS=	rubygem-minitar>=0.6.0:archivers/rubygem-minitar \
		rubygem-powerbar>=1.0:devel/rubygem-powerbar

NO_ARCH=	yes
USE_RUBY=	yes
USES=		gem

PLIST_FILES=	bin/minitar

.include <bsd.port.mk>
