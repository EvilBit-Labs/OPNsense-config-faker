from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Pfsyncpeerip(BaseModel):
    class Meta:
        name = "pfsyncpeerip"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
