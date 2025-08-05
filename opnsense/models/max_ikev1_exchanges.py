from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class MaxIkev1Exchanges(BaseModel):
    class Meta:
        name = "max_ikev1_exchanges"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
