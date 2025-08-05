from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Gatewayv6(BaseModel):
    class Meta:
        name = "gatewayv6"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
