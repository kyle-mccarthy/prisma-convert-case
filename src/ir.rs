use anyhow::{Error, Result};
use datamodel::{
    dml::{WithDatabaseName, WithName},
    parse_schema, render_datamodel_to_string, Configuration, Datamodel,
};
use heck::{CamelCase, MixedCase};

impl From<(Configuration, Datamodel)> for Intermediate {
    fn from((config, datamodel): (Configuration, Datamodel)) -> Self {
        Self { config, datamodel }
    }
}

#[derive(Debug)]
pub struct Intermediate {
    config: Configuration,
    datamodel: Datamodel,
}

impl Intermediate {
    pub fn parse(input: &str) -> Result<Self> {
        Ok(parse_schema(input).map_err(Error::msg)?.into())
    }

    pub fn render(&self) -> String {
        render_datamodel_to_string(&self.datamodel, Some(&self.config))
    }

    pub fn transform_names(&mut self) {
        self.datamodel.models_mut().for_each(|model| {
            let name = model.name().to_string();

            model.set_name(&name.to_camel_case());
            model.set_database_name(Some(name));

            model.fields_mut().for_each(|field| {
                let name = field.name().to_string();
                field.set_name(&name.to_mixed_case());
                field.set_database_name(Some(name));
            });

            model.indices.iter_mut().for_each(|index| {
                if let Some(db_name) = index.db_name.as_ref() {
                    index.name = Some(db_name.to_mixed_case());
                }
                index.fields.iter_mut().for_each(|field| {
                    let name = field.name.to_mixed_case();
                    field.name = name;
                });
            });
        });
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_simple_parse() {
        let schema = r#"
            generator client {
              provider = "prisma-client-js"
            }

            datasource db {
              provider = "mysql"
              url      = env("DB_URL")
            }

            model auth_otp {
              id                   Int       @id @default(autoincrement())
              user_id              Int
              otp                  String    @unique(map: "otp") @db.VarChar(255)
              destroyed_at         DateTime? @db.Timestamp(0)
              redeemed_at          DateTime? @db.Timestamp(0)
              user_agent_issued_to String?   @db.VarChar(255)
              ip_address_issued_to String?   @db.VarChar(255)
              user_agent_redeemed  String?   @db.VarChar(255)
              ip_address_redeemed  String?   @db.VarChar(255)
              updated_at           DateTime  @default(now()) @db.Timestamp(0)
              created_at           DateTime? @default(now()) @db.Timestamp(0)

              @@index([user_id], map: "user_id")
            }
        "#;

        let expected = indoc! {r#"
            model AuthOtp {
              id                Int       @id @default(autoincrement()) @map("id")
              userId            Int       @map("user_id")
              otp               String    @unique(map: "otp") @map("otp") @db.VarChar(255)
              destroyedAt       DateTime? @map("destroyed_at") @db.Timestamp(0)
              redeemedAt        DateTime? @map("redeemed_at") @db.Timestamp(0)
              userAgentIssuedTo String?   @map("user_agent_issued_to") @db.VarChar(255)
              ipAddressIssuedTo String?   @map("ip_address_issued_to") @db.VarChar(255)
              userAgentRedeemed String?   @map("user_agent_redeemed") @db.VarChar(255)
              ipAddressRedeemed String?   @map("ip_address_redeemed") @db.VarChar(255)
              updatedAt         DateTime  @default(now()) @map("updated_at") @db.Timestamp(0)
              createdAt         DateTime? @default(now()) @map("created_at") @db.Timestamp(0)

              @@index([userId], map: "user_id")
              @@map("auth_otp")
            }
        "#};

        let mut ir = Intermediate::parse(schema).unwrap();

        ir.transform_names();

        let output = ir.render();

        assert_eq!(expected, output);
    }
}
