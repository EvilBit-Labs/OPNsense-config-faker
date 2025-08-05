from typing import Optional

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.apikeys import Apikeys
from opnsense.models.authorizedkeys import Authorizedkeys
from opnsense.models.descr import Descr
from opnsense.models.expires import Expires
from opnsense.models.groupname import Groupname
from opnsense.models.ipsecpsk import Ipsecpsk
from opnsense.models.name import Name
from opnsense.models.otp_seed import OtpSeed
from opnsense.models.password import Password
from opnsense.models.scope import Scope
from opnsense.models.uid import Uid

__NAMESPACE__ = "https://opnsense.org/config"


class User(BaseModel):
    class Meta:
        name = "user"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    name: Name = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    descr: Descr = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    scope: Scope = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    groupname: Groupname = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    password: Password = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    uid: Uid = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    apikeys: Apikeys | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    expires: Expires | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    authorizedkeys: Authorizedkeys | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    ipsecpsk: Ipsecpsk | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    otp_seed: OtpSeed | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
