from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.aggressivensec import Aggressivensec
from opnsense.models.cachemaxnegativettl import Cachemaxnegativettl
from opnsense.models.cachemaxttl import Cachemaxttl
from opnsense.models.cacheminttl import Cacheminttl
from opnsense.models.discardtimeout import Discardtimeout
from opnsense.models.dnssecstripped import Dnssecstripped
from opnsense.models.extendedstatistics import Extendedstatistics
from opnsense.models.hideidentity import Hideidentity
from opnsense.models.hideversion import Hideversion
from opnsense.models.incomingnumtcp import Incomingnumtcp
from opnsense.models.infracachenumhosts import Infracachenumhosts
from opnsense.models.infrahostttl import Infrahostttl
from opnsense.models.infrakeepprobing import Infrakeepprobing
from opnsense.models.insecuredomain import Insecuredomain
from opnsense.models.jostletimeout import Jostletimeout
from opnsense.models.loglocalactions import Loglocalactions
from opnsense.models.logqueries import Logqueries
from opnsense.models.logreplies import Logreplies
from opnsense.models.logservfail import Logservfail
from opnsense.models.logtagqueryreply import Logtagqueryreply
from opnsense.models.logverbosity import Logverbosity
from opnsense.models.msgcachesize import Msgcachesize
from opnsense.models.numqueriesperthread import Numqueriesperthread
from opnsense.models.outgoingnumtcp import Outgoingnumtcp
from opnsense.models.outgoingrange import Outgoingrange
from opnsense.models.prefetch import Prefetch
from opnsense.models.prefetchkey import Prefetchkey
from opnsense.models.privateaddress import Privateaddress
from opnsense.models.privatedomain import Privatedomain
from opnsense.models.qnameminstrict import Qnameminstrict
from opnsense.models.rrsetcachesize import Rrsetcachesize
from opnsense.models.serveexpired import Serveexpired
from opnsense.models.serveexpiredclienttimeout import Serveexpiredclienttimeout
from opnsense.models.serveexpiredreplyttl import Serveexpiredreplyttl
from opnsense.models.serveexpiredttl import Serveexpiredttl
from opnsense.models.serveexpiredttlreset import Serveexpiredttlreset
from opnsense.models.unwantedreplythreshold import Unwantedreplythreshold
from opnsense.models.valloglevel import Valloglevel

__NAMESPACE__ = "https://opnsense.org/config"


class Advanced(BaseModel):
    class Meta:
        name = "advanced"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    hideidentity: Hideidentity = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    hideversion: Hideversion = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    prefetch: Prefetch = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    prefetchkey: Prefetchkey = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    dnssecstripped: Dnssecstripped = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    aggressivensec: Aggressivensec = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    serveexpired: Serveexpired = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    serveexpiredreplyttl: Serveexpiredreplyttl = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    serveexpiredttl: Serveexpiredttl = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    serveexpiredttlreset: Serveexpiredttlreset = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    serveexpiredclienttimeout: Serveexpiredclienttimeout = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    qnameminstrict: Qnameminstrict = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    extendedstatistics: Extendedstatistics = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    logqueries: Logqueries = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    logreplies: Logreplies = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    logtagqueryreply: Logtagqueryreply = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    logservfail: Logservfail = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    loglocalactions: Loglocalactions = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    logverbosity: Logverbosity = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    valloglevel: Valloglevel = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    privatedomain: Privatedomain = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    privateaddress: Privateaddress = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    insecuredomain: Insecuredomain = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    msgcachesize: Msgcachesize = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    rrsetcachesize: Rrsetcachesize = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    outgoingnumtcp: Outgoingnumtcp = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    incomingnumtcp: Incomingnumtcp = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    numqueriesperthread: Numqueriesperthread = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    outgoingrange: Outgoingrange = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    jostletimeout: Jostletimeout = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    discardtimeout: Discardtimeout = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    cachemaxttl: Cachemaxttl = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    cachemaxnegativettl: Cachemaxnegativettl = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    cacheminttl: Cacheminttl = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    infrahostttl: Infrahostttl = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    infrakeepprobing: Infrakeepprobing = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    infracachenumhosts: Infracachenumhosts = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    unwantedreplythreshold: Unwantedreplythreshold = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
