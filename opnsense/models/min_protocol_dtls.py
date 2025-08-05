from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class MinProtocolDtls(BaseModel):
    class Meta:
        name = "MinProtocol_DTLS"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
