from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class UserDefinedRules(BaseModel):
    class Meta:
        name = "userDefinedRules"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
